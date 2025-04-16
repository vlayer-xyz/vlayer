use alloy_primitives::{B256, BlockNumber};
use alloy_rlp::encode_fixed_size;
use block_header::EvmBlockHeader;
use bytes::Bytes;
use common::Hashable;
use derivative::Derivative;
use mpt::{Sha2Node, Sha2Trie};
use serde::{Deserialize, Serialize};
use thiserror::Error;

#[derive(Debug, Error, PartialEq)]
pub enum BlockTrieError {
    #[error("failed to get block hash: {0}")]
    GetBlockHashFailed(BlockNumber),
    #[error("block hash mismatch: expected: {expected} != actual: {actual}")]
    BlockHashMismatch { expected: B256, actual: B256 },
    #[error("failed to insert block hash: {0}")]
    InsertBlockHashFailed(#[from] mpt::MptError),
}

pub type BlockTrieResult<T> = Result<T, BlockTrieError>;

#[derive(Debug, Clone, Default, Derivative, Serialize, Deserialize, PartialEq, Eq)]
pub struct BlockTrie(Sha2Trie);

impl BlockTrie {
    pub fn init(block: impl AsRef<dyn EvmBlockHeader>) -> BlockTrieResult<Self> {
        let block = block.as_ref();
        let mut trie = Self(Sha2Trie::new());
        trie.insert_unchecked(block.number(), &block.hash_slow())?;
        Ok(trie)
    }

    /// `new_rightmost_block` is the header of the block to be appended, i.e. the next
    /// block after the block with highest number currently stored in the trie
    pub fn append_single(
        &mut self,
        new_rightmost_block: impl AsRef<dyn EvmBlockHeader>,
    ) -> BlockTrieResult<()> {
        let new_rightmost_block = new_rightmost_block.as_ref();
        let parent_block_idx = new_rightmost_block.number() - 1;
        let parent_block_hash = self
            .get(parent_block_idx)
            .ok_or(BlockTrieError::GetBlockHashFailed(parent_block_idx))?;
        if parent_block_hash != *new_rightmost_block.parent_hash() {
            return Err(BlockTrieError::BlockHashMismatch {
                expected: parent_block_hash,
                actual: *new_rightmost_block.parent_hash(),
            });
        }
        self.insert_unchecked(new_rightmost_block.number(), &new_rightmost_block.hash_slow())?;
        Ok(())
    }

    /// `old_leftmost_block` is the header of the block with lowest number currently
    /// stored in the trie
    pub fn prepend_single(
        &mut self,
        old_leftmost_block: impl AsRef<dyn EvmBlockHeader>,
    ) -> BlockTrieResult<()> {
        let old_leftmost_block = old_leftmost_block.as_ref();
        let old_leftmost_block_hash = self
            .get(old_leftmost_block.number())
            .ok_or(BlockTrieError::GetBlockHashFailed(old_leftmost_block.number()))?;
        if old_leftmost_block_hash != *old_leftmost_block.hash_slow() {
            return Err(BlockTrieError::BlockHashMismatch {
                expected: old_leftmost_block_hash,
                actual: old_leftmost_block.hash_slow(),
            });
        }
        self.insert_unchecked(old_leftmost_block.number() - 1, old_leftmost_block.parent_hash())?;
        Ok(())
    }

    pub fn append<B>(&mut self, blocks: impl Iterator<Item = B>) -> BlockTrieResult<()>
    where
        B: AsRef<dyn EvmBlockHeader>,
    {
        for block in blocks {
            self.append_single(block)?;
        }
        Ok(())
    }

    pub fn prepend<B>(
        &mut self,
        blocks: impl DoubleEndedIterator<Item = B>,
        mut old_leftmost_block: B,
    ) -> BlockTrieResult<()>
    where
        B: AsRef<dyn EvmBlockHeader>,
    {
        for block in blocks.rev() {
            self.prepend_single(old_leftmost_block)?;

            old_leftmost_block = block;
        }
        Ok(())
    }

    pub const fn from_unchecked(mpt: Sha2Trie) -> Self {
        Self(mpt)
    }

    pub fn get(&self, block_number: BlockNumber) -> Option<B256> {
        let key = Self::encode_key(block_number);
        self.0.get(key).map(B256::from_slice)
    }

    pub fn insert_unchecked(
        &mut self,
        block_number: BlockNumber,
        hash: &B256,
    ) -> BlockTrieResult<()> {
        let key = Self::encode_key(block_number);
        self.0.insert(key, hash)?;
        Ok(())
    }

    pub fn size(&self) -> usize {
        self.0.size()
    }

    fn encode_key(block_number: BlockNumber) -> impl AsRef<[u8]> {
        encode_fixed_size(&block_number)
    }

    pub fn into_root(self) -> Sha2Node {
        self.0.0
    }
}

impl Hashable for BlockTrie {
    fn hash_slow(&self) -> B256 {
        self.0.hash_slow()
    }
}

impl IntoIterator for &BlockTrie {
    type IntoIter = std::vec::IntoIter<Bytes>;
    type Item = Bytes;

    fn into_iter(self) -> Self::IntoIter {
        self.0.into_iter()
    }
}

#[cfg(feature = "test-utils")]
#[allow(clippy::unwrap_used)]
pub fn mock_block_trie(blocks: std::ops::RangeInclusive<BlockNumber>) -> BlockTrie {
    let mut trie = BlockTrie::default();
    for header in block_header::test_utils::mock_block_headers(blocks) {
        trie.insert_unchecked(header.number(), &header.hash_slow())
            .unwrap();
    }
    trie
}

#[cfg(test)]
mod tests {
    use anyhow::Result;
    use chain_test_utils::{
        fake_block_with_correct_parent_hash as block,
        fake_block_with_incorrect_parent_hash as block_with_incorrect_hash,
    };

    use super::{BlockTrie, BlockTrieError};

    mod append_single {
        use super::*;

        #[test]
        fn correct_hash() -> Result<()> {
            let block_zero = block(0);
            let block_one = block(1);
            let mut trie = BlockTrie::init(block_zero)?;

            trie.append_single(&block_one)?;

            assert_eq!(trie.get(1).unwrap(), block_one.hash_slow());
            Ok(())
        }

        #[test]
        fn incorrect_hash() -> Result<()> {
            let block_zero = block(0);
            let block_one = block_with_incorrect_hash(1);
            let mut trie = BlockTrie::init(block_zero)?;

            let result = trie.append_single(block_one);

            assert!(matches!(result.unwrap_err(), BlockTrieError::BlockHashMismatch { .. }));
            Ok(())
        }
    }

    mod prepend_single {
        use super::*;

        #[test]
        fn correct_hash() -> Result<()> {
            let block_one = block(1);
            let mut trie = BlockTrie::init(&block_one)?;
            let old_leftmost_block = block_one;

            trie.prepend_single(old_leftmost_block)?;

            assert_eq!(trie.get(0).unwrap(), block(0).hash_slow());
            Ok(())
        }

        #[test]
        fn incorrect_hash() -> Result<()> {
            let block_one = block(1);
            let mut trie = BlockTrie::init(block_one)?;

            let result = trie.prepend_single(block_with_incorrect_hash(1));

            assert!(matches!(result.unwrap_err(), BlockTrieError::BlockHashMismatch { .. }));
            Ok(())
        }
    }
}

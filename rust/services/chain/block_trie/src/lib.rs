use alloy_primitives::B256;
use alloy_rlp::encode_fixed_size;
use block_header::EvmBlockHeader;
use bytes::Bytes;
use derivative::Derivative;
use mpt::{MerkleTrie, Node};
use risc0_zkp::verify::VerificationError;
use risc0_zkvm::{sha::Digest, Receipt};
use serde::{Deserialize, Serialize};
use thiserror::Error;

#[derive(Debug, Error, Derivative)]
#[derivative(PartialEq)]
pub enum BlockTrieError {
    #[error("proof verification failed: {0}")]
    ProofVerificationFailed(#[from] VerificationError),
    #[error("failed to deserialize receipt: {0}")]
    DeserializeReceiptFailed(
        #[from]
        #[derivative(PartialEq = "ignore")]
        bincode::Error,
    ),
    #[error("failed to deserialize journal: {0}")]
    DeserializeJournalFailed(#[from] risc0_zkvm::serde::Error),
    #[error("elf id mismatch: expected: {expected} != decoded: {decoded}")]
    ElfIdMismatch { expected: Digest, decoded: Digest },
    #[error("mpt root mismatch: expected: {expected} != decoded: {decoded}")]
    MptRootMismatch { expected: B256, decoded: B256 },
    #[error("failed to get block hash: {0}")]
    GetBlockHashFailed(u64),
    #[error("block hash mismatch: expected: {expected} != actual: {actual}")]
    BlockHashMismatch { expected: B256, actual: B256 },
    #[error("failed to insert block hash: {0}")]
    InsertBlockHashFailed(#[from] mpt::MptError),
}

pub type BlockTrieResult<T> = Result<T, BlockTrieError>;

#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize, Deserialize)]
pub struct BlockTrie(MerkleTrie);

impl BlockTrie {
    pub fn init(block: &dyn EvmBlockHeader) -> BlockTrieResult<Self> {
        let mut trie = Self(MerkleTrie::new());
        trie.insert_unchecked(block.number(), &block.hash_slow())?;
        Ok(trie)
    }

    /// `new_rightmost_block` is the header of the block to be appended, i.e. the next
    /// block after the block with highest number currently stored in the trie
    pub fn append(&mut self, new_rightmost_block: &dyn EvmBlockHeader) -> BlockTrieResult<()> {
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
    pub fn prepend(&mut self, old_leftmost_block: &dyn EvmBlockHeader) -> BlockTrieResult<()> {
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

    pub fn from_unchecked(mpt: MerkleTrie) -> Self {
        Self(mpt)
    }

    pub fn from_proof(
        mpt: MerkleTrie,
        zk_proof: &Bytes,
        guest_id: impl Into<Digest>,
    ) -> BlockTrieResult<Self> {
        let guest_id = guest_id.into();
        let receipt: Receipt = bincode::deserialize(zk_proof)?;
        receipt.verify(guest_id)?;

        let (proven_root, elf_id): (B256, Digest) = receipt.journal.decode()?;

        if elf_id != guest_id {
            return Err(BlockTrieError::ElfIdMismatch {
                expected: guest_id,
                decoded: elf_id,
            });
        }
        if mpt.hash_slow() != proven_root {
            return Err(BlockTrieError::MptRootMismatch {
                expected: mpt.hash_slow(),
                decoded: proven_root,
            });
        }

        Ok(BlockTrie(mpt))
    }

    pub fn get(&self, block_number: u64) -> Option<B256> {
        let key = Self::encode_key(block_number);
        self.0.get(key).map(B256::from_slice)
    }

    pub fn insert_unchecked(&mut self, block_number: u64, hash: &B256) -> BlockTrieResult<()> {
        let key = Self::encode_key(block_number);
        self.0.insert(key, hash)?;
        Ok(())
    }

    pub fn hash_slow(&self) -> B256 {
        self.0.hash_slow()
    }

    fn encode_key(block_number: u64) -> impl AsRef<[u8]> {
        encode_fixed_size(&block_number)
    }

    pub fn into_root(self) -> Node {
        self.0 .0
    }
}

impl<'a> IntoIterator for &'a BlockTrie {
    type IntoIter = std::vec::IntoIter<Bytes>;
    type Item = Bytes;

    fn into_iter(self) -> Self::IntoIter {
        self.0.into_iter()
    }
}

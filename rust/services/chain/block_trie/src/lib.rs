use alloy_primitives::B256;
use alloy_rlp::encode_fixed_size;
use block_header::EvmBlockHeader;
use bytes::Bytes;
use mpt::{MerkleTrie, Node};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize, Deserialize)]
pub struct BlockTrie(MerkleTrie);

impl BlockTrie {
    pub fn init(block: &dyn EvmBlockHeader) -> Self {
        let mut trie = Self(MerkleTrie::new());
        trie.insert_unchecked(block.number(), &block.hash_slow());
        trie
    }

    /// `new_rightmost_block` is the header of the block to be appended, i.e. the next
    /// block after the block with highest number currently stored in the trie
    pub fn append(&mut self, new_rightmost_block: &dyn EvmBlockHeader) {
        let parent_block_idx = new_rightmost_block.number() - 1;
        let parent_block_hash = self
            .get(parent_block_idx)
            .expect("failed to get parent block hash");
        assert_eq!(parent_block_hash, *new_rightmost_block.parent_hash(), "block hash mismatch");
        self.insert_unchecked(new_rightmost_block.number(), &new_rightmost_block.hash_slow());
    }

    /// `old_leftmost_block` is the header of the block with lowest number currently
    /// stored in the trie
    pub fn prepend(&mut self, old_leftmost_block: &dyn EvmBlockHeader) {
        let old_leftmost_block_hash = self
            .get(old_leftmost_block.number())
            .expect("failed to get old leftmost block hash");
        assert_eq!(old_leftmost_block_hash, old_leftmost_block.hash_slow(), "block hash mismatch");
        self.insert_unchecked(old_leftmost_block.number() - 1, old_leftmost_block.parent_hash());
    }

    pub fn from_unchecked(mpt: MerkleTrie) -> Self {
        Self(mpt)
    }

    pub fn get(&self, block_number: u64) -> Option<B256> {
        let key = Self::encode_key(block_number);
        self.0.get(key).map(B256::from_slice)
    }

    pub fn insert_unchecked(&mut self, block_number: u64, hash: &B256) {
        let key = Self::encode_key(block_number);
        self.0.insert(key, hash).expect("insert block number");
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

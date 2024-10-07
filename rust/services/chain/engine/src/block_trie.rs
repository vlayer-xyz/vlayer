use alloy_primitives::B256;
use alloy_rlp::encode_fixed_size;
use bytes::Bytes;
use mpt::{MerkleTrie, Node};

pub struct BlockTrie(MerkleTrie);

impl BlockTrie {
    pub fn new() -> Self {
        Self(MerkleTrie::new())
    }

    pub fn get(&self, block_number: u64) -> Option<&[u8]> {
        let key = Self::encode_key(block_number);
        self.0.get(key)
    }

    pub fn insert(&mut self, block_number: u64, hash: &B256) {
        let key = Self::encode_key(block_number);
        self.0.insert(key, hash).expect("insert block number");
    }

    pub fn hash_slow(&self) -> B256 {
        self.0.hash_slow()
    }

    pub fn encode_key(block_number: u64) -> impl AsRef<[u8]> {
        encode_fixed_size(&block_number)
    }

    pub fn to_rlp_nodes(&self) -> impl Iterator<Item = Bytes> + '_ {
        self.0.to_rlp_nodes()
    }

    pub fn root_node(self) -> Node {
        self.0 .0
    }
}

impl From<MerkleTrie> for BlockTrie {
    fn from(mpt: MerkleTrie) -> Self {
        Self(mpt)
    }
}

use alloy_primitives::B256;
use alloy_rlp::encode_fixed_size;
use bytes::Bytes;
use mpt::{MerkleTrie, Node};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize, Deserialize)]
pub struct BlockTrie(MerkleTrie);

impl BlockTrie {
    pub fn new() -> Self {
        Self(MerkleTrie::new())
    }

    pub fn from_mpt(mpt: MerkleTrie) -> Self {
        Self(mpt)
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

    pub fn into_root(self) -> Node {
        self.0 .0
    }
}

impl From<MerkleTrie> for BlockTrie {
    fn from(mpt: MerkleTrie) -> Self {
        Self(mpt)
    }
}

impl FromIterator<(u64, B256)> for BlockTrie {
    fn from_iter<I>(iter: I) -> Self
    where
        I: IntoIterator<Item = (u64, B256)>,
    {
        let mut trie = BlockTrie::new();
        for (key, value) in iter {
            trie.insert(key, &value)
        }
        trie
    }
}

impl<'a> IntoIterator for &'a BlockTrie {
    type IntoIter = std::vec::IntoIter<Bytes>;
    type Item = Bytes;

    fn into_iter(self) -> Self::IntoIter {
        self.0.into_iter()
    }
}

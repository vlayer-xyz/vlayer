use alloy_primitives::bytes::Bytes;
use block_trie::BlockTrie;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize, Deserialize)]
pub struct ChainProof {
    pub proof: Bytes,
    pub block_trie: BlockTrie,
}

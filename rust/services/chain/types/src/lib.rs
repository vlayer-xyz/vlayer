use alloy_primitives::bytes::Bytes;
use block_trie::BlockTrie;
use serde::{Deserialize, Serialize};

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct ChainProof {
    pub proof: Bytes,
    pub block_trie: BlockTrie,
}

use alloy_primitives::bytes::Bytes;
use block_trie::BlockTrie;
use mpt::{MerkleTrie, ParseNodeError};
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
pub struct ChainProof {
    pub proof: Bytes,
    pub block_trie: BlockTrie,
}

#[derive(Serialize, Default, Deserialize, Debug, PartialEq)]
pub struct RpcChainProof {
    pub proof: Bytes,
    pub nodes: Vec<Bytes>,
}

impl TryFrom<RpcChainProof> for ChainProof {
    type Error = ParseNodeError;

    fn try_from(rpc_chain_proof: RpcChainProof) -> Result<Self, Self::Error> {
        let block_trie =
            BlockTrie::from_unchecked(MerkleTrie::from_rlp_nodes(rpc_chain_proof.nodes)?);
        Ok(Self {
            proof: rpc_chain_proof.proof,
            block_trie,
        })
    }
}

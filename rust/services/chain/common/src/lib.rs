use alloy_primitives::bytes::Bytes;
use block_trie::BlockTrie;
use mpt::{MerkleTrie, ParseNodeError};
use serde::{Deserialize, Serialize};
use serde_with::{hex::Hex, serde_as};

#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
pub struct ChainProof {
    pub proof: Bytes,
    pub block_trie: BlockTrie,
}

#[serde_as]
#[derive(Serialize, Default, Deserialize, Debug, PartialEq)]
pub struct RpcChainProof {
    #[serde_as(as = "Hex")]
    pub proof: Bytes,
    #[serde_as(as = "Vec<Hex>")]
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

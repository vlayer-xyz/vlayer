use std::ops::RangeInclusive;

use alloy_primitives::BlockNumber;
use block_trie::{BlockTrie, ProofVerificationError};
use bytes::Bytes;
use chain_guest_wrapper::RISC0_CHAIN_GUEST_ID;
use mpt::MerkleTrie;

use crate::ChainProof;

pub struct UnverifiedChainTrie {
    pub block_range: RangeInclusive<BlockNumber>,
    pub trie: MerkleTrie,
    pub zk_proof: Bytes,
}

impl UnverifiedChainTrie {
    pub const fn new(
        block_range: RangeInclusive<BlockNumber>,
        trie: MerkleTrie,
        zk_proof: Bytes,
    ) -> Self {
        Self {
            block_range,
            trie,
            zk_proof,
        }
    }
}

// `trie` held by this struct is proven by `zk_proof` to be correctly constructed
pub struct ChainTrie {
    pub block_range: RangeInclusive<BlockNumber>,
    pub trie: BlockTrie,
    pub zk_proof: ChainProof,
}

impl ChainTrie {
    pub fn new(
        block_range: RangeInclusive<BlockNumber>,
        trie: BlockTrie,
        zk_proof: impl Into<ChainProof>,
    ) -> Self {
        Self {
            block_range,
            trie,
            zk_proof: zk_proof.into(),
        }
    }
}

impl TryFrom<UnverifiedChainTrie> for ChainTrie {
    type Error = ProofVerificationError;

    fn try_from(
        UnverifiedChainTrie {
            block_range,
            trie,
            zk_proof,
        }: UnverifiedChainTrie,
    ) -> Result<Self, Self::Error> {
        let block_trie =
            BlockTrie::from_mpt_verifying_the_proof(trie, &zk_proof, RISC0_CHAIN_GUEST_ID)?;
        Ok(ChainTrie::new(block_range, block_trie, &zk_proof))
    }
}

use std::ops::RangeInclusive;

use alloy_primitives::BlockNumber;
use block_trie::BlockTrie;
use derive_new::new;
use mpt::MerkleTrie;
use traits::Hashable;

use crate::{receipt::ProofVerificationError, ChainProofReceipt};

#[derive(new)]
pub struct UnverifiedChainTrie {
    pub block_range: RangeInclusive<BlockNumber>,
    pub trie: MerkleTrie,
    pub zk_proof: ChainProofReceipt,
}

// `trie` held by this struct is proven by `zk_proof` to be correctly constructed
pub struct ChainTrie {
    pub block_range: RangeInclusive<BlockNumber>,
    pub trie: BlockTrie,
    pub zk_proof: ChainProofReceipt,
}

impl ChainTrie {
    pub fn new(
        block_range: RangeInclusive<BlockNumber>,
        trie: BlockTrie,
        zk_proof: impl Into<ChainProofReceipt>,
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
        zk_proof.verify(trie.hash_slow())?;
        let block_trie = BlockTrie::from_unchecked(trie);
        Ok(ChainTrie::new(block_range, block_trie, zk_proof))
    }
}

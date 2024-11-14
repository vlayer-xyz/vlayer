use block_trie::BlockTrie;
use chain_common::{ChainProofReceipt, ProofVerificationError};
use chain_guest_wrapper::GUEST;
use common::Hashable;
use derive_new::new;
use mpt::MerkleTrie;
use u64_range::NonEmptyRange;

#[derive(new)]
pub struct UnverifiedChainTrie {
    pub block_range: NonEmptyRange,
    pub trie: MerkleTrie,
    pub zk_proof: ChainProofReceipt,
}

// `trie` held by this struct is proven by `zk_proof` to be correctly constructed
pub struct ChainTrie {
    pub block_range: NonEmptyRange,
    pub trie: BlockTrie,
    pub zk_proof: ChainProofReceipt,
}

impl ChainTrie {
    pub fn new(
        block_range: NonEmptyRange,
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
        zk_proof.verify(trie.hash_slow(), GUEST.id)?;
        let block_trie = BlockTrie::from_unchecked(trie);
        Ok(ChainTrie::new(block_range, block_trie, zk_proof))
    }
}

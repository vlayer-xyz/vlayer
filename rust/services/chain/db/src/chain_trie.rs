use block_trie::KeccakBlockTrie as BlockTrie;
use chain_common::{ChainProofReceipt, ProofVerificationError};
use common::{GuestElf, Hashable};
use derive_new::new;
use mpt::KeccakMerkleTrie as MerkleTrie;
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

pub fn verify_chain_trie(
    unverified: UnverifiedChainTrie,
    elf: &GuestElf,
) -> Result<ChainTrie, ProofVerificationError> {
    let UnverifiedChainTrie {
        block_range,
        trie,
        zk_proof,
    } = unverified;
    zk_proof.verify(trie.hash_slow(), elf.id)?;
    let block_trie = BlockTrie::from_unchecked(trie);
    Ok(ChainTrie::new(block_range, block_trie, zk_proof))
}

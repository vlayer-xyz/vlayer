use block_trie::BlockTrie;
use derive_more::{Deref, From, Into};
use derive_new::new;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ChainProof {
    pub receipt: ChainProofReceipt,
    pub block_trie: BlockTrie,
}

impl ChainProof {
    pub fn as_ref(&self) -> ChainProofRef<'_, '_> {
        ChainProofRef::new(&self.receipt, &self.block_trie)
    }
}

#[derive(Clone, Debug, new)]
pub struct ChainProofRef<'receipt, 'trie> {
    pub(crate) receipt: &'receipt u64, // TODO: fake type
    pub(crate) block_trie: &'trie BlockTrie,
}

#[derive(Debug, Clone, From, Into, Deref, Serialize, Deserialize)]
pub struct ChainProofReceipt(u64); // TODO: fake type

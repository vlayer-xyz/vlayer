use alloy_primitives::{BlockNumber, ChainId};
use block_header::EvmBlockHeader;
use block_trie::BlockTrie;
use bytes::Bytes;
use common::{Hashable, Method};
use derive_more::{Deref, From, Into};
use derive_new::new;
use mpt::{ParseNodeError, Sha2Trie, Sha256, reorder_root_first};
use risc0_zkvm::{
    AssumptionReceipt, FakeReceipt, Receipt, ReceiptClaim, serde::to_vec, sha::Digest,
};
use serde::{Deserialize, Serialize};
use serde_with::{hex::Hex, serde_as};
use thiserror::Error;

pub mod verifier;

#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize, Deserialize, new)]
pub struct GetChainProof {
    pub(crate) chain_id: ChainId,
    pub(crate) block_numbers: Vec<BlockNumber>,
}

impl Method for GetChainProof {
    const METHOD_NAME: &str = "v_getChainProof";
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ChainProof {
    pub receipt: ChainProofReceipt,
    pub block_trie: BlockTrie,
}

#[cfg(feature = "testing")]
impl PartialEq for ChainProof {
    fn eq(&self, other: &Self) -> bool {
        self.receipt == other.receipt && self.block_trie == other.block_trie
    }
}

#[cfg(feature = "testing")]
#[allow(clippy::expect_used)]
/// Mock chain proof with arbitrary block numbers. Does **not** generate valid block hashes.
pub fn mock_chain_proof(block_numbers: impl IntoIterator<Item = BlockNumber>) -> ChainProof {
    use alloy_primitives::BlockHash;

    let mut block_trie = BlockTrie::default();
    for block_num in block_numbers {
        block_trie
            .insert_unchecked(block_num, &BlockHash::default())
            .expect("insert_unchecked failed");
    }
    ChainProof {
        receipt: mock_chain_proof_receipt(),
        block_trie,
    }
}

#[cfg(feature = "testing")]
/// Mock chain proof with continuous block range. Does generate valid block hashes.
pub fn mock_chain_proof_with_hashes(
    block_numbers: std::ops::RangeInclusive<BlockNumber>,
) -> ChainProof {
    use block_trie::mock_block_trie;

    ChainProof {
        receipt: mock_chain_proof_receipt(),
        block_trie: mock_block_trie(block_numbers),
    }
}

impl ChainProof {
    pub fn as_ref(&self) -> ChainProofRef<'_, '_> {
        ChainProofRef::new(&self.receipt, &self.block_trie)
    }
}

#[derive(Clone, Debug, new)]
pub struct ChainProofRef<'receipt, 'trie> {
    pub(crate) receipt: &'receipt Receipt,
    pub(crate) block_trie: &'trie BlockTrie,
}

#[derive(Debug, Clone, From, Into, Deref, Serialize, Deserialize)]
pub struct ChainProofReceipt(Receipt);

#[cfg(feature = "testing")]
impl PartialEq for ChainProofReceipt {
    fn eq(&self, other: &Self) -> bool {
        self.0.journal == other.0.journal
            && self.0.metadata == other.0.metadata
            && to_vec(&self.0.inner) == to_vec(&other.0.inner) // InnerReceipt doesn't implement PartialEq
    }
}

#[cfg(feature = "testing")]
pub fn mock_chain_proof_receipt() -> ChainProofReceipt {
    let claim = ReceiptClaim::ok(Digest::default(), vec![]);
    let inner = risc0_zkvm::InnerReceipt::Fake(FakeReceipt::new(claim));
    Receipt::new(inner, vec![]).into()
}

impl TryFrom<&ChainProofReceipt> for Bytes {
    type Error = bincode::Error;

    fn try_from(receipt: &ChainProofReceipt) -> Result<Self, Self::Error> {
        Ok(bincode::serialize(&receipt.0)?.into())
    }
}

impl TryFrom<&Bytes> for ChainProofReceipt {
    type Error = bincode::Error;

    fn try_from(bytes: &Bytes) -> Result<Self, Self::Error> {
        Ok(ChainProofReceipt(bincode::deserialize(bytes)?))
    }
}

impl From<ChainProofReceipt> for AssumptionReceipt {
    fn from(receipt: ChainProofReceipt) -> Self {
        receipt.0.into()
    }
}

#[serde_as]
#[derive(Default, Clone, Debug, PartialEq, Serialize, Deserialize, new)]
pub struct RpcChainProof {
    #[serde_as(as = "Hex")]
    pub proof: Bytes,
    #[serde_as(as = "Vec<Hex>")]
    pub nodes: Vec<Bytes>,
}

impl TryFrom<&ChainProof> for RpcChainProof {
    type Error = bincode::Error;

    fn try_from(chain_proof: &ChainProof) -> Result<Self, Self::Error> {
        let proof = (&chain_proof.receipt).try_into()?;
        let root_hash = chain_proof.block_trie.hash_slow();
        let nodes = reorder_root_first::<_, Sha256>(chain_proof.block_trie.into_iter(), root_hash);
        Ok(Self { proof, nodes })
    }
}

#[derive(Debug, Error)]
pub enum ParseProofError {
    #[error("failed to deserialize receipt: {0}")]
    DeserializeReceiptFailed(#[from] bincode::Error),
    #[error("failed to parse block trie: {0}")]
    Mpt(#[from] ParseNodeError),
}

impl TryFrom<RpcChainProof> for ChainProof {
    type Error = ParseProofError;

    fn try_from(rpc_chain_proof: RpcChainProof) -> Result<Self, Self::Error> {
        let block_trie =
            BlockTrie::from_unchecked(Sha2Trie::from_rlp_nodes(rpc_chain_proof.nodes)?);
        let receipt = (&rpc_chain_proof.proof).try_into()?;
        Ok(Self {
            receipt,
            block_trie,
        })
    }
}

#[allow(clippy::expect_used, clippy::unwrap_used)]
pub fn fake_proof_result(
    guest_id: Digest,
    block_headers: impl IntoIterator<Item = Box<dyn EvmBlockHeader>>,
) -> RpcChainProof {
    let mut block_trie = BlockTrie::default();
    for header in block_headers {
        block_trie
            .insert_unchecked(header.number(), &header.hash_slow())
            .expect("insert block failed");
    }
    let root_hash = block_trie.hash_slow();
    let proof_output = to_vec(&(root_hash, guest_id)).unwrap();
    let journal: Vec<u8> = bytemuck::cast_slice(&proof_output).into();
    let inner: FakeReceipt<ReceiptClaim> =
        FakeReceipt::<ReceiptClaim>::new(ReceiptClaim::ok(guest_id, journal.clone()));
    let receipt = Receipt::new(risc0_zkvm::InnerReceipt::Fake(inner), journal);
    let encoded_proof = bincode::serialize(&receipt).unwrap().into();
    let nodes: Vec<Bytes> = block_trie.into_iter().collect();

    RpcChainProof::new(encoded_proof, nodes)
}

#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize, Deserialize, new)]
pub struct GetSyncStatus {
    pub chain_id: ChainId,
}

impl Method for GetSyncStatus {
    const METHOD_NAME: &str = "v_getSyncStatus";
}

#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize, Deserialize, new)]
pub struct SyncStatus {
    pub first_block: BlockNumber,
    pub last_block: BlockNumber,
}

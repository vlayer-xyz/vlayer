use alloy_primitives::{BlockNumber, ChainId, B256};
use block_header::EvmBlockHeader;
use block_trie::BlockTrie;
use bytes::Bytes;
use common::{Hashable, Method};
use derivative::Derivative;
use derive_more::{AsRef, Deref, From, Into};
use derive_new::new;
use mpt::{ParseNodeError, Sha2Trie};
use risc0_zkp::verify::VerificationError;
use risc0_zkvm::{
    serde::to_vec, sha::Digest, AssumptionReceipt, FakeReceipt, Receipt, ReceiptClaim,
};
use serde::{Deserialize, Serialize};
use serde_with::{hex::Hex, serde_as};
use thiserror::Error;

pub mod verifier;

#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize, Deserialize, new)]
pub struct GetChainProof {
    pub chain_id: ChainId,
    pub block_numbers: Vec<BlockNumber>,
}

impl Method for GetChainProof {
    const METHOD_NAME: &str = "v_getChainProof";
}

#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize, AsRef)]
pub struct ChainProof {
    #[as_ref]
    pub proof: Bytes,
    #[as_ref]
    pub block_trie: BlockTrie,
}

#[derive(Debug, Error, Derivative)]
#[derivative(PartialEq)]
pub enum ProofVerificationError {
    #[error("proof verification failed: {0}")]
    ProofVerificationFailed(#[from] VerificationError),
    #[error("failed to deserialize receipt: {0}")]
    DeserializeReceiptFailed(
        #[from]
        #[derivative(PartialEq = "ignore")]
        bincode::Error,
    ),
    #[error("failed to deserialize journal: {0}")]
    DeserializeJournalFailed(#[from] risc0_zkvm::serde::Error),
    #[error("illegal elf id: {decoded} not in {expected:?}")]
    IllegalElfId {
        expected: Box<[Digest]>,
        decoded: Digest,
    },
    #[error("mpt root mismatch: expected: {expected} != decoded: {decoded}")]
    MptRootMismatch { expected: B256, decoded: B256 },
}

#[derive(Debug, Clone, From, Into, Deref)]
pub struct ChainProofReceipt(Receipt);

impl ChainProofReceipt {
    pub fn verify(
        &self,
        expected_hash: B256,
        chain_guest_ids: Box<[Digest]>,
    ) -> Result<(), ProofVerificationError> {
        let receipt = &self.0;
        let (proven_root, elf_id): (B256, Digest) = receipt.journal.decode()?;

        if !chain_guest_ids.iter().any(|id| id == &elf_id) {
            return Err(ProofVerificationError::IllegalElfId {
                expected: chain_guest_ids,
                decoded: elf_id,
            });
        }

        if expected_hash != proven_root {
            return Err(ProofVerificationError::MptRootMismatch {
                expected: expected_hash,
                decoded: proven_root,
            });
        }
        receipt.verify(elf_id)?;
        Ok(())
    }
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

impl TryFrom<&ChainProof> for ChainProofReceipt {
    type Error = bincode::Error;

    fn try_from(chain_proof: &ChainProof) -> Result<Self, Self::Error> {
        let bytes: &Bytes = chain_proof.as_ref();
        bytes.try_into()
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

impl TryFrom<RpcChainProof> for ChainProof {
    type Error = ParseNodeError;

    fn try_from(rpc_chain_proof: RpcChainProof) -> Result<Self, Self::Error> {
        let block_trie =
            BlockTrie::from_unchecked(Sha2Trie::from_rlp_nodes(rpc_chain_proof.nodes)?);
        Ok(Self {
            proof: rpc_chain_proof.proof,
            block_trie,
        })
    }
}

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

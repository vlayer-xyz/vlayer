use alloy_primitives::{BlockNumber, ChainId, B256};
use block_trie::KeccakBlockTrie as BlockTrie;
use bytes::Bytes;
use common::Method;
use derivative::Derivative;
use derive_more::{AsRef, Deref, From, Into};
use derive_new::new;
use mpt::{KeccakMerkleTrie, ParseNodeError};
use risc0_zkp::verify::VerificationError;
use risc0_zkvm::{sha::Digest, AssumptionReceipt, Receipt};
use serde::{Deserialize, Serialize};
use serde_with::{hex::Hex, serde_as};
use thiserror::Error;

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
    #[error("elf id mismatch: expected: {expected} != decoded: {decoded}")]
    ElfIdMismatch { expected: Digest, decoded: Digest },
    #[error("mpt root mismatch: expected: {expected} != decoded: {decoded}")]
    MptRootMismatch { expected: B256, decoded: B256 },
}

#[derive(Debug, Clone, From, Into, Deref)]
pub struct ChainProofReceipt(Receipt);

impl ChainProofReceipt {
    pub fn verify(
        &self,
        expected_hash: B256,
        expected_elf_id: Digest,
    ) -> Result<(), ProofVerificationError> {
        let receipt = &self.0;
        receipt.verify(expected_elf_id)?;
        let (proven_root, elf_id): (B256, Digest) = receipt.journal.decode()?;

        if elf_id != expected_elf_id {
            return Err(ProofVerificationError::ElfIdMismatch {
                expected: expected_elf_id,
                decoded: elf_id,
            });
        }
        if expected_hash != proven_root {
            return Err(ProofVerificationError::MptRootMismatch {
                expected: expected_hash,
                decoded: proven_root,
            });
        }
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
            BlockTrie::from_unchecked(KeccakMerkleTrie::from_rlp_nodes(rpc_chain_proof.nodes)?);
        Ok(Self {
            proof: rpc_chain_proof.proof,
            block_trie,
        })
    }
}

#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize, Deserialize, new)]
pub struct GetSyncStatus {
    pub chain_id: ChainId,
}

impl Method for GetSyncStatus {
    const METHOD_NAME: &str = "v_sync_status";
}

#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize, Deserialize, new)]
pub struct SyncStatus {
    pub first_block: BlockNumber,
    pub last_block: BlockNumber,
}

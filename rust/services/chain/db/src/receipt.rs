use alloy_primitives::B256;
use bytes::Bytes;
use chain_guest_wrapper::RISC0_CHAIN_GUEST_ID;
use derivative::Derivative;
use derive_more::{Deref, From, Into};
use risc0_zkp::verify::VerificationError;
use risc0_zkvm::{sha::Digest, AssumptionReceipt, Receipt};
use thiserror::Error;

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
    pub fn verify(&self, expected_hash: B256) -> Result<(), ProofVerificationError> {
        let receipt = &self.0;
        let guest_id = RISC0_CHAIN_GUEST_ID.into();
        receipt.verify(RISC0_CHAIN_GUEST_ID)?;
        let (proven_root, elf_id): (B256, Digest) = receipt.journal.decode()?;

        if elf_id != guest_id {
            return Err(ProofVerificationError::ElfIdMismatch {
                expected: guest_id,
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

impl TryFrom<ChainProofReceipt> for Bytes {
    type Error = bincode::Error;

    fn try_from(receipt: ChainProofReceipt) -> Result<Self, Self::Error> {
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

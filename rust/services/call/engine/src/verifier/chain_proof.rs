use alloy_primitives::B256;
#[cfg(test)]
use auto_impl::auto_impl;
use block_header::Hashable;
use chain_common::ChainProof;
use risc0_zkp::verify::VerificationError;
use risc0_zkvm::sha::Digest;
use static_assertions::assert_obj_safe;

use super::ZkpVerifier;

#[derive(thiserror::Error, Debug)]
pub enum ChainProofError {
    #[error("Receipt deserialization error: {0}")]
    Receipt(#[from] bincode::Error),
    #[error("ZK verification error: {0}")]
    Zk(#[from] VerificationError),
    #[error("Journal decoding error: {0}")]
    Journal(#[from] risc0_zkvm::serde::Error),
    #[error("ELF ID mismatch: expected={expected} got={got}")]
    ElfId { expected: Digest, got: Digest },
    #[error("Root hash mismatch: proven={proven} actual={actual}")]
    RootHash { proven: B256, actual: B256 },
}

mod seal {
    // This trait prevents adding new implementations of ChainProofVerifier
    pub trait Sealed {}

    // Useful to mock verifier in tests
    #[cfg(test)]
    impl<F: Fn(&super::ChainProof) -> Result<(), super::ChainProofError>> Sealed for F {}
}

#[cfg_attr(test, auto_impl(Fn))]
pub trait ChainProofVerifier: seal::Sealed + Send + Sync + 'static {
    fn verify(&self, proof: &ChainProof) -> Result<(), ChainProofError>;
}

assert_obj_safe!(ChainProofVerifier);

pub struct ZkChainProofVerifier {
    chain_guest_id: Digest,
    zk_verifier: Box<dyn ZkpVerifier>,
}

impl ZkChainProofVerifier {
    #[must_use]
    pub fn new(chain_guest_id: impl Into<Digest>, zk_verifier: impl ZkpVerifier) -> Self {
        Self {
            chain_guest_id: chain_guest_id.into(),
            zk_verifier: Box::new(zk_verifier),
        }
    }
}

impl seal::Sealed for ZkChainProofVerifier {}
impl ChainProofVerifier for ZkChainProofVerifier {
    fn verify(&self, proof: &ChainProof) -> Result<(), ChainProofError> {
        let receipt = bincode::deserialize(&proof.proof)?;
        self.zk_verifier.verify(&receipt, self.chain_guest_id)?;
        let (proven_root, elf_id) = receipt.journal.decode()?;
        let root_hash = proof.block_trie.hash_slow();
        if elf_id != self.chain_guest_id {
            return Err(ChainProofError::ElfId {
                expected: self.chain_guest_id,
                got: elf_id,
            });
        }
        if proven_root != root_hash {
            return Err(ChainProofError::RootHash {
                proven: proven_root,
                actual: root_hash,
            });
        }
        Ok(())
    }
}

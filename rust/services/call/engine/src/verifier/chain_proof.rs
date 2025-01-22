use alloy_primitives::B256;
use block_header::Hashable;
use chain_common::{ChainProof, ChainProofReceipt};
use risc0_zkp::verify::VerificationError;
use risc0_zkvm::sha::Digest;
use static_assertions::assert_obj_safe;

use super::{impl_verifier_for_fn, sealed_trait, verifier_trait, zk_proof};

#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error("Receipt deserialization error: {0}")]
    Receipt(#[from] bincode::Error),
    #[error("ZK verification error: {0}")]
    Zk(#[from] VerificationError),
    #[error("Journal decoding error: {0}")]
    Journal(#[from] risc0_zkvm::serde::Error),
    #[error("ELF ID mismatch: expected={expected:?} got={got}")]
    ElfId {
        expected: Box<[Digest]>,
        got: Digest,
    },
    #[error("Root hash mismatch: proven={proven} actual={actual}")]
    RootHash { proven: B256, actual: B256 },
}

pub type Result = std::result::Result<(), Error>;

sealed_trait!((&ChainProof));
verifier_trait!((proof: &ChainProof) -> Result);
impl_verifier_for_fn!((proof: &ChainProof) -> Result);

pub struct Verifier<ZK: zk_proof::IVerifier> {
    chain_guest_ids: Box<[Digest]>,
    zk_verifier: ZK,
}

impl<ZK: zk_proof::IVerifier> Verifier<ZK> {
    #[must_use]
    pub fn new(chain_guest_ids: impl IntoIterator<Item = Digest>, zk_verifier: ZK) -> Self {
        Self {
            chain_guest_ids: chain_guest_ids.into_iter().collect(),
            zk_verifier,
        }
    }
}

impl<ZK: zk_proof::IVerifier> seal::Sealed for Verifier<ZK> {}
impl<ZK: zk_proof::IVerifier> IVerifier for Verifier<ZK> {
    fn verify(&self, proof: &ChainProof) -> Result {
        let receipt: ChainProofReceipt = (&proof.proof).try_into()?;
        let (proven_root, elf_id) = receipt.journal.decode()?;
        if !self.chain_guest_ids.iter().any(|id| id == &elf_id) {
            return Err(Error::ElfId {
                expected: self.chain_guest_ids.clone(),
                got: elf_id,
            });
        }

        let root_hash = proof.block_trie.hash_slow();
        if proven_root != root_hash {
            return Err(Error::RootHash {
                proven: proven_root,
                actual: root_hash,
            });
        }

        self.zk_verifier.verify(&receipt, elf_id)?;
        Ok(())
    }
}

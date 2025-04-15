use alloy_primitives::B256;
use block_header::Hashable;
use common::{sealed_with_test_mock, verifier::zk_proof};
use derivative::Derivative;
use risc0_zkp::verify::VerificationError;
use risc0_zkvm::sha::Digest;

use super::ChainProofRef;

#[derive(thiserror::Error, Debug, Derivative)]
#[derivative(PartialEq)]
pub enum Error {
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
sealed_with_test_mock!(IVerifier (proof_ref: ChainProofRef<'_, '_>) -> Result);

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
    fn verify(&self, proof_ref: ChainProofRef<'_, '_>) -> Result {
        let (proven_root, elf_id) = proof_ref.receipt.journal.decode()?;
        if !self.chain_guest_ids.iter().any(|id| id == &elf_id) {
            return Err(Error::ElfId {
                expected: self.chain_guest_ids.clone(),
                got: elf_id,
            });
        }

        let root_hash = proof_ref.block_trie.hash_slow();
        if proven_root != root_hash {
            return Err(Error::RootHash {
                proven: proven_root,
                actual: root_hash,
            });
        }

        self.zk_verifier.verify(proof_ref.receipt, elf_id)?;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use block_trie::{BlockTrie, mock_block_trie};
    use common::verifier::zk_proof;
    use risc0_zkvm::{FakeReceipt, InnerReceipt, Receipt, ReceiptClaim, serde::to_vec};

    use super::*;
    use crate::{ChainProof, ChainProofReceipt};

    const CHAIN_GUEST_ID: Digest = Digest::new([0, 0, 0, 0, 0, 0, 0, 1]);
    const INVALID_ROOT_HASH: B256 = B256::ZERO;
    const INVALID_ELF_ID: Digest = Digest::ZERO;

    fn mock_receipt(journal: Vec<u8>) -> ChainProofReceipt {
        let inner: FakeReceipt<ReceiptClaim> =
            FakeReceipt::<ReceiptClaim>::new(ReceiptClaim::ok(CHAIN_GUEST_ID, journal.clone()));

        Receipt::new(InnerReceipt::Fake(inner), journal).into()
    }

    fn mock_chain_proof(block_trie: BlockTrie, journal: Vec<u8>) -> ChainProof {
        let receipt = mock_receipt(journal);
        ChainProof {
            receipt,
            block_trie,
        }
    }

    fn mock_journal(root_hash: B256, elf_id: Digest) -> Vec<u8> {
        let journal = to_vec(&(root_hash, elf_id)).unwrap();
        bytemuck::cast_slice(&journal).into()
    }

    const fn proof_ok(_: &Receipt, _: Digest) -> zk_proof::Result {
        Ok(())
    }

    const fn proof_invalid(_: &Receipt, _: Digest) -> zk_proof::Result {
        Err(zk_proof::Error::InvalidProof)
    }

    #[test]
    fn ok() {
        let verifier = Verifier::new([CHAIN_GUEST_ID], proof_ok);
        let block_trie = mock_block_trie(0..=1);
        let journal = mock_journal(block_trie.hash_slow(), CHAIN_GUEST_ID);
        let proof = mock_chain_proof(block_trie, journal);
        verifier.verify(proof.as_ref()).expect("verify failed");
    }

    #[test]
    fn zk_verification_fail() {
        let verifier = Verifier::new([CHAIN_GUEST_ID], proof_invalid);
        #[allow(clippy::reversed_empty_ranges)]
        let block_trie = mock_block_trie(1..=0);
        let journal = mock_journal(block_trie.hash_slow(), CHAIN_GUEST_ID);
        let proof = mock_chain_proof(block_trie, journal);
        let res = verifier.verify(proof.as_ref());
        assert!(matches!(res.unwrap_err(), Error::Zk(zk_proof::Error::InvalidProof)));
    }

    #[test]
    fn invalid_root_hash() {
        let verifier = Verifier::new([CHAIN_GUEST_ID], proof_ok);
        let block_trie = mock_block_trie(0..=1);
        let _root_hash = block_trie.hash_slow();
        let journal = mock_journal(INVALID_ROOT_HASH, CHAIN_GUEST_ID);
        let proof = mock_chain_proof(block_trie, journal);
        let res = verifier.verify(proof.as_ref());
        assert!(matches!(
            res.unwrap_err(),
            Error::RootHash {
                proven: INVALID_ROOT_HASH,
                actual: _root_hash,
            }
        ));
    }

    #[test]
    fn invalid_elf_id() {
        let verifier = Verifier::new([CHAIN_GUEST_ID], proof_ok);
        let block_trie = mock_block_trie(0..=1);
        let journal = mock_journal(block_trie.hash_slow(), INVALID_ELF_ID);
        let proof = mock_chain_proof(block_trie, journal);
        let res = verifier.verify(proof.as_ref());
        let _expected_ids = vec![CHAIN_GUEST_ID].into_boxed_slice();
        assert!(matches!(
            res.unwrap_err(),
            Error::ElfId {
                expected: _expected_ids,
                got: INVALID_ELF_ID
            }
        ));
    }
}

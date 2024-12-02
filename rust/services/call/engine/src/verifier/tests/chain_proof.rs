use alloy_rlp::Bytes;
use chain_common::ChainProof;
use risc0_zkvm::Receipt;

use super::*;
use crate::verifier::{
    chain_proof::{Error, Verifier, ZkVerifier},
    zk_proof,
};

const INVALID_ROOT_HASH: B256 = B256::ZERO;
const INVALID_ELF_ID: Digest = Digest::ZERO;

fn mock_receipt(journal: Vec<u8>) -> Receipt {
    let inner: FakeReceipt<ReceiptClaim> =
        FakeReceipt::<ReceiptClaim>::new(ReceiptClaim::ok(CHAIN_GUEST_ID, journal.clone()));

    Receipt::new(InnerReceipt::Fake(inner), journal)
}

fn mock_chain_proof(block_trie: BlockTrie, journal: Vec<u8>) -> ChainProof {
    let proof = bincode::serialize(&mock_receipt(journal)).unwrap().into();
    ChainProof { proof, block_trie }
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
fn ok() -> anyhow::Result<()> {
    let verifier = ZkVerifier::new(CHAIN_GUEST_ID, proof_ok);
    let block_trie = mock_block_trie(0..=1);
    let journal = mock_journal(block_trie.hash_slow(), CHAIN_GUEST_ID);
    let proof = mock_chain_proof(block_trie, journal);
    verifier.verify(&proof)?;

    Ok(())
}

#[test]
fn invalid_receipt() {
    let verifier = ZkVerifier::new(CHAIN_GUEST_ID, proof_ok);
    let proof = ChainProof {
        proof: Bytes::new(),
        block_trie: Default::default(),
    };
    let res = verifier.verify(&proof);
    assert!(matches!(res.unwrap_err(), Error::Receipt(..)));
}

#[test]
fn zk_verification_fail() {
    let verifier = ZkVerifier::new(CHAIN_GUEST_ID, proof_invalid);
    #[allow(clippy::reversed_empty_ranges)]
    let block_trie = mock_block_trie(1..=0);
    let proof = mock_chain_proof(block_trie, vec![]);
    let res = verifier.verify(&proof);
    assert!(matches!(res.unwrap_err(), Error::Zk(zk_proof::Error::InvalidProof)));
}

#[test]
fn invalid_root_hash() {
    let verifier = ZkVerifier::new(CHAIN_GUEST_ID, proof_ok);
    let block_trie = mock_block_trie(0..=1);
    let _root_hash = block_trie.hash_slow();
    let journal = mock_journal(INVALID_ROOT_HASH, CHAIN_GUEST_ID);
    let proof = mock_chain_proof(block_trie, journal);
    let res = verifier.verify(&proof);
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
    let verifier = ZkVerifier::new(CHAIN_GUEST_ID, proof_ok);
    let block_trie = mock_block_trie(0..=1);
    let journal = mock_journal(block_trie.hash_slow(), INVALID_ELF_ID);
    let proof = mock_chain_proof(block_trie, journal);
    let res = verifier.verify(&proof);
    assert!(matches!(
        res.unwrap_err(),
        Error::ElfId {
            expected: CHAIN_GUEST_ID,
            got: INVALID_ELF_ID
        }
    ));
}

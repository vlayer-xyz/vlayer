use std::iter;

use alloy_primitives::B256;
use block_header::EvmBlockHeader;
use block_trie::BlockTrie;
use chain_common::{ChainProofRef, verifier::IVerifier};
use common::{Hashable, verifier::zk_proof::GuestVerifier};
use risc0_zkp::core::digest::Digest;
use risc0_zkvm::Receipt;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub enum Input {
    Initialize {
        elf_id: Digest,
        block: Box<dyn EvmBlockHeader>,
    },
    AppendPrepend {
        elf_id: Digest,
        prepend_blocks: Vec<Box<dyn EvmBlockHeader>>,
        append_blocks: Vec<Box<dyn EvmBlockHeader>>,
        old_leftmost_block: Box<dyn EvmBlockHeader>,
        prev_zk_proof: Box<Receipt>,
        block_trie: BlockTrie,
    },
}

#[allow(clippy::expect_used)]
fn initialize(elf_id: Digest, block: impl AsRef<dyn EvmBlockHeader>) -> (B256, Digest) {
    let trie = BlockTrie::init(block).expect("init failed");
    (trie.hash_slow(), elf_id)
}

#[allow(clippy::expect_used)]
fn verify_previous_proof(
    prev_zk_proof: &Receipt,
    block_trie: &BlockTrie,
    current_elf_id: &Digest,
    old_elf_ids: impl IntoIterator<Item = Digest>,
) {
    let proof_ref = ChainProofRef::new(prev_zk_proof, block_trie);
    let chain_guest_ids = old_elf_ids.into_iter().chain(iter::once(*current_elf_id));
    let zk_verifier = GuestVerifier;
    let verifier = chain_common::verifier::Verifier::new(chain_guest_ids, zk_verifier);
    verifier
        .verify(proof_ref)
        .expect("previous proof verification failed");
}

#[allow(clippy::expect_used)]
fn append_prepend(
    elf_id: Digest,
    prepend_blocks: impl DoubleEndedIterator<Item = Box<dyn EvmBlockHeader>>,
    append_blocks: impl Iterator<Item = Box<dyn EvmBlockHeader>>,
    old_leftmost_block: Box<dyn EvmBlockHeader>,
    prev_zk_proof: &Receipt,
    old_elf_ids: impl IntoIterator<Item = Digest>,
    mut block_trie: BlockTrie,
) -> (B256, Digest) {
    verify_previous_proof(prev_zk_proof, &block_trie, &elf_id, old_elf_ids);

    block_trie.append(append_blocks).expect("append failed");
    block_trie
        .prepend(prepend_blocks, old_leftmost_block)
        .expect("prepend failed");

    (block_trie.hash_slow(), elf_id)
}

#[allow(clippy::unused_async)]
pub async fn main(input: Input, old_elf_ids: impl IntoIterator<Item = Digest>) -> (B256, Digest) {
    match input {
        Input::Initialize { elf_id, block } => initialize(elf_id, block),
        Input::AppendPrepend {
            elf_id,
            prepend_blocks,
            append_blocks,
            old_leftmost_block,
            block_trie,
            prev_zk_proof,
        } => append_prepend(
            elf_id,
            prepend_blocks.into_iter(),
            append_blocks.into_iter(),
            old_leftmost_block,
            &prev_zk_proof,
            old_elf_ids,
            block_trie,
        ),
    }
}

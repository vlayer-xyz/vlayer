use alloy_primitives::B256;
use block_header::EvmBlockHeader;
use block_trie::BlockTrie;
use common::Hashable;
use risc0_zkp::core::digest::Digest;
use risc0_zkvm::{guest::env, serde::from_slice, Receipt};
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

fn initialize(elf_id: Digest, block: impl AsRef<dyn EvmBlockHeader>) -> (B256, Digest) {
    let trie = BlockTrie::init(block).expect("init failed");
    (trie.hash_slow(), elf_id)
}

fn verify_previous_proof(
    prev_zk_proof: &Receipt,
    block_trie: &BlockTrie,
    current_elf_id: &Digest,
    old_elf_ids: impl IntoIterator<Item = Digest>,
) {
    let (prev_proven_hash, prev_elf_id): (B256, Digest) =
        from_slice(prev_zk_proof.journal.as_ref())
            .expect("failed to deserialize previous ZK proof");
    assert!(
        &prev_elf_id == current_elf_id || old_elf_ids.into_iter().any(|id| id == prev_elf_id),
        "invalid ELF ID in previous ZK proof"
    );
    assert_eq!(
        prev_proven_hash,
        block_trie.hash_slow(),
        "invalid root hash in previous ZK proof"
    );
    env::verify(prev_elf_id, prev_zk_proof.journal.as_ref())
        .expect("failed to verify previous ZK proof");
}

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

use alloy_primitives::B256;
use block_header::EvmBlockHeader;
use block_trie::BlockTrie;
use risc0_zkp::core::digest::Digest;
use risc0_zkvm::{guest::env, serde::to_vec};
use serde::{Deserialize, Serialize};
use traits::Hashable;

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
        block_trie: BlockTrie,
    },
}

fn initialize(elf_id: Digest, block: impl AsRef<dyn EvmBlockHeader>) -> (B256, Digest) {
    let trie = BlockTrie::init(block).expect("init failed");
    (trie.hash_slow(), elf_id)
}

fn append_prepend(
    elf_id: Digest,
    prepend_blocks: impl DoubleEndedIterator<Item = Box<dyn EvmBlockHeader>>,
    append_blocks: impl Iterator<Item = Box<dyn EvmBlockHeader>>,
    old_leftmost_block: Box<dyn EvmBlockHeader>,
    mut block_trie: BlockTrie,
) -> (B256, Digest) {
    let expected_prev_proof_output =
        to_vec(&(block_trie.hash_slow(), elf_id)).expect("failed to serialize");
    env::verify(elf_id, &expected_prev_proof_output).expect("failed to verify previous ZK proof");

    block_trie.append(append_blocks).expect("append failed");
    block_trie
        .prepend(prepend_blocks, old_leftmost_block)
        .expect("prepend failed");

    (block_trie.hash_slow(), elf_id)
}

#[allow(clippy::unused_async)]
pub async fn main(input: Input) -> (B256, Digest) {
    match input {
        Input::Initialize { elf_id, block } => initialize(elf_id, block),
        Input::AppendPrepend {
            elf_id,
            prepend_blocks,
            append_blocks,
            old_leftmost_block,
            block_trie,
        } => append_prepend(
            elf_id,
            prepend_blocks.into_iter(),
            append_blocks.into_iter(),
            old_leftmost_block,
            block_trie,
        ),
    }
}

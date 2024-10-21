use alloy_primitives::B256;
use block_header::EvmBlockHeader;
use block_trie::BlockTrie;
use risc0_zkp::core::digest::Digest;
use risc0_zkvm::{guest::env, serde::to_vec};
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
        block_trie: BlockTrie,
    },
}

fn initialize(elf_id: Digest, block: &dyn EvmBlockHeader) -> (B256, Digest) {
    let trie = BlockTrie::init(block).expect("init failed");
    (trie.hash_slow(), elf_id)
}

fn append_prepend(
    elf_id: Digest,
    prepend_blocks: impl DoubleEndedIterator<Item = Box<dyn EvmBlockHeader>>,
    append_blocks: impl Iterator<Item = Box<dyn EvmBlockHeader>>,
    mut old_leftmost_block: Box<dyn EvmBlockHeader>,
    mut block_trie: BlockTrie,
) -> (B256, Digest) {
    let prev_proof_output = to_vec(&(block_trie.hash_slow(), elf_id)).expect("failed to serialize");
    env::verify(elf_id, &prev_proof_output).expect("failed to verify previous ZK proof");

    for block in append_blocks {
        block_trie.append(&*block).expect("append failed");
    }
    for block in prepend_blocks.rev() {
        block_trie
            .prepend(&*old_leftmost_block)
            .expect("prepend failed");

        old_leftmost_block = block;
    }

    (block_trie.hash_slow(), elf_id)
}

pub fn main(input: Input) -> (B256, Digest) {
    match input {
        Input::Initialize { elf_id, block } => initialize(elf_id, &*block),
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

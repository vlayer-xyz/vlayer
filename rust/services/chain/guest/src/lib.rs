use alloy_primitives::B256;
use block_header::EvmBlockHeader;
use block_trie::BlockTrie;
use chain_common::{GuestVerifier, Risc0Verifier};
use risc0_zkp::core::digest::Digest;
use risc0_zkvm::serde::to_vec;
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

fn initialize(block: &dyn EvmBlockHeader) -> B256 {
    let trie = BlockTrie::init(block).expect("init failed");
    trie.hash_slow()
}

fn append_prepend(
    elf_id: Digest,
    verifier: &ChainGuestVerifier,
    prepend_blocks: impl DoubleEndedIterator<Item = Box<dyn EvmBlockHeader>>,
    append_blocks: impl Iterator<Item = Box<dyn EvmBlockHeader>>,
    old_leftmost_block: Box<dyn EvmBlockHeader>,
    mut block_trie: BlockTrie,
) -> B256 {
    verifier.verify_prev_output(block_trie.hash_slow(), elf_id);

    block_trie.append(append_blocks).expect("append failed");
    block_trie
        .prepend(prepend_blocks, old_leftmost_block)
        .expect("prepend failed");

    block_trie.hash_slow()
}

#[allow(clippy::unused_async)]
pub async fn main(input: Input, verifier: &ChainGuestVerifier) -> (B256, Digest) {
    match input {
        Input::Initialize { elf_id, block } => (initialize(&*block), elf_id),
        Input::AppendPrepend {
            elf_id,
            prepend_blocks,
            append_blocks,
            old_leftmost_block,
            block_trie,
        } => (
            append_prepend(
                elf_id,
                verifier,
                prepend_blocks.into_iter(),
                append_blocks.into_iter(),
                old_leftmost_block,
                block_trie,
            ),
            elf_id,
        ),
    }
}

pub struct ChainGuestVerifier(Box<dyn GuestVerifier>);

impl ChainGuestVerifier {
    pub fn new_risc0() -> Self {
        Self::new(Risc0Verifier)
    }

    pub fn new(verifier: impl GuestVerifier) -> Self {
        Self(Box::new(verifier))
    }

    pub fn verify_prev_output(&self, prev_hash: B256, elf_id: Digest) {
        let prev_proof_output = to_vec(&(prev_hash, elf_id)).expect("failed to serialize");
        // It is safe to cast `prev_proof_output` to &[u8] as it is just a byte sequence written as 32-bit words
        self.0
            .verify(elf_id, bytemuck::cast_slice(&prev_proof_output));
    }
}

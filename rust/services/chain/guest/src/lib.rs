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

fn initialize(block: &dyn EvmBlockHeader) -> B256 {
    let trie = BlockTrie::init(block).expect("init failed");
    trie.hash_slow()
}

fn append_prepend(
    elf_id: Digest,
    verifier: &ChainGuestVerifier,
    prepend_blocks: impl DoubleEndedIterator<Item = Box<dyn EvmBlockHeader>>,
    append_blocks: impl Iterator<Item = Box<dyn EvmBlockHeader>>,
    mut old_leftmost_block: Box<dyn EvmBlockHeader>,
    mut block_trie: BlockTrie,
) -> B256 {
    verifier.verify_prev_output(block_trie.hash_slow(), elf_id);

    for block in append_blocks {
        block_trie.append(&*block).expect("append failed");
    }
    for block in prepend_blocks.rev() {
        block_trie
            .prepend(&*old_leftmost_block)
            .expect("prepend failed");

        old_leftmost_block = block;
    }

    block_trie.hash_slow()
}

pub fn main(input: Input, verifier: &ChainGuestVerifier) -> (B256, Digest) {
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
    pub fn new(verifier: impl GuestVerifier) -> Self {
        Self(Box::new(verifier))
    }

    pub fn verify_prev_output(&self, prev_hash: B256, elf_id: Digest) {
        let prev_proof_output = to_vec(&(prev_hash, elf_id)).expect("failed to serialize");
        self.0
            .verify(elf_id, bytemuck::cast_slice(&prev_proof_output));
    }
}

pub trait GuestVerifier: Send + Sync + 'static {
    /// Verify ZK proof from guest code. This entails that verifying the receipt of the currently
    /// executing guest code will also guarantee that the given proof is valid.
    /// Panic if proof is invalid.
    fn verify(&self, elf_id: Digest, proof: &[u8]);
}

pub struct Risc0Verifier;

impl GuestVerifier for Risc0Verifier {
    fn verify(&self, elf_id: Digest, proof: &[u8]) {
        env::verify(elf_id, proof).expect("infallible");
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[allow(dead_code)]
    struct MockVerifier {
        verification_ok: bool,
    }

    impl MockVerifier {
        #[allow(dead_code)]
        pub const fn new(verification_ok: bool) -> Self {
            Self { verification_ok }
        }
    }

    impl GuestVerifier for MockVerifier {
        fn verify(&self, _elf_id: Digest, _proof: &[u8]) {
            assert!(self.verification_ok, "proof verification failed")
        }
    }
}

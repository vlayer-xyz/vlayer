use block_header::EvmBlockHeader;
use bytes::Bytes;
use risc0_zkp::core::digest::Digest;
use serde::{Deserialize, Serialize};

mod block_trie;
pub use block_trie::BlockTrie;

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
        mpt_nodes: Box<[Bytes]>,
    },
}

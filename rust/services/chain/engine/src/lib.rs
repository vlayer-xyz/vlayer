use block_header::EvmBlockHeader;
use risc0_zkp::core::digest::Digest;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub enum Input {
    Initialize {
        block: Box<dyn EvmBlockHeader>,
        elf_id: Digest,
    },
}

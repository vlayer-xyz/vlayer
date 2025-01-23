use std::path::PathBuf;

use alloy_primitives::ChainId;
use common::GuestElf;
use ethers::types::BlockNumber as BlockTag;
use host_utils::ProofMode;
use risc0_zkvm::sha::Digest;

use super::strategy::{AppendStrategy, PrependStrategy};

#[derive(Debug)]
pub struct HostConfig {
    pub rpc_url: String,
    pub chain_id: ChainId,
    pub proof_mode: ProofMode,
    pub db_path: PathBuf,
    pub elf: GuestElf,
    pub chain_guest_ids: Box<[Digest]>,
    pub start_block: BlockTag,
    pub prepend_strategy: PrependStrategy,
    pub append_strategy: AppendStrategy,
}

use std::collections::HashMap;

use alloy_primitives::ChainId;
use host_utils::ProofMode;

pub const DEFAULT_MAX_CALLDATA_SIZE: usize = 5 * 1024 * 1024; // 5 MB

pub struct HostConfig {
    pub rpc_urls: HashMap<ChainId, String>,
    pub start_chain_id: ChainId,
    pub proof_mode: ProofMode,
    pub chain_proof_url: String,
    pub max_calldata_size: usize,
}

impl Default for HostConfig {
    fn default() -> Self {
        Self {
            rpc_urls: HashMap::default(),
            start_chain_id: ChainId::default(),
            proof_mode: ProofMode::default(),
            chain_proof_url: String::default(),
            max_calldata_size: DEFAULT_MAX_CALLDATA_SIZE,
        }
    }
}

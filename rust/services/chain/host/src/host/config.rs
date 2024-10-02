use std::collections::HashMap;

use alloy_primitives::ChainId;
use host_utils::ProofMode;

#[derive(Default)]
pub struct HostConfig {
    pub rpc_urls: HashMap<ChainId, String>,
    pub proof_mode: ProofMode,
}

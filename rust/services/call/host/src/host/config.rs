use std::collections::HashMap;

use alloy_primitives::ChainId;

#[derive(Default, Clone, Copy)]
pub enum ProofMode {
    #[default]
    Fake,
    Groth16,
}

#[derive(Default)]
pub struct HostConfig {
    pub rpc_urls: HashMap<ChainId, String>,
    pub start_chain_id: ChainId,
    pub proof_mode: ProofMode,
}

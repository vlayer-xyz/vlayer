use alloy_primitives::ChainId;
use host_utils::ProofMode;

#[derive(Default, Debug)]
pub struct HostConfig {
    pub rpc_url: String,
    pub chain_id: ChainId,
    pub proof_mode: ProofMode,
    pub db_path: String,
}

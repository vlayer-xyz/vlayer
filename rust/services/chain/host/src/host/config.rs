use host_utils::ProofMode;

#[derive(Default)]
pub struct HostConfig {
    pub rpc_url: String,
    pub proof_mode: ProofMode,
}

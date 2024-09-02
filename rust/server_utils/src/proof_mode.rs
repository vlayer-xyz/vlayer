use serde::{Deserialize, Serialize};

use call_host::host::config::ProofMode as HostProofMode;

#[derive(Debug, Clone, Deserialize, Serialize)]
pub enum ProofMode {
    Groth16,
    Fake,
}

impl From<ProofMode> for HostProofMode {
    fn from(val: ProofMode) -> Self {
        match val {
            ProofMode::Groth16 => HostProofMode::Groth16,
            ProofMode::Fake => HostProofMode::Fake,
        }
    }
}

impl ProofMode {
    pub fn set_risc0_flag(&self) {
        let value = match self {
            ProofMode::Groth16 => "0",
            ProofMode::Fake => "1",
        };
        std::env::set_var("RISC0_DEV_MODE", value);
    }
}

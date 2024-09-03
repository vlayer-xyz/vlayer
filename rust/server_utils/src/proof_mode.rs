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

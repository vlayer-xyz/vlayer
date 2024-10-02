use host_utils::ProofMode as HostProofMode;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Deserialize, Serialize, Copy)]
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

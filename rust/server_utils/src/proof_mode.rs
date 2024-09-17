use serde::{Deserialize, Serialize};

use host_utils::ProofMode as HostProofMode;

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

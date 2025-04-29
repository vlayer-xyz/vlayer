use host_utils::ProofMode as HostProofMode;
use serde::{Deserialize, Serialize};
use strum::{Display, EnumString};

#[derive(
    Debug, Clone, Deserialize, Serialize, Copy, PartialEq, Eq, Display, EnumString, Default,
)]
#[strum(ascii_case_insensitive)]
#[strum(serialize_all = "lowercase")]
pub enum ProofMode {
    #[default]
    Fake,
    Groth16,
}

impl From<ProofMode> for HostProofMode {
    fn from(val: ProofMode) -> Self {
        match val {
            ProofMode::Groth16 => HostProofMode::Groth16,
            ProofMode::Fake => HostProofMode::Fake,
        }
    }
}

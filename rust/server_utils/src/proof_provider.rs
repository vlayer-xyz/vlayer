use host_utils::ProofProvider as HostProofProvider;
use serde::{Deserialize, Serialize};
use strum::{Display, EnumString};

#[derive(
    Debug, Clone, Deserialize, Serialize, Copy, PartialEq, Eq, Display, EnumString, Default,
)]
#[strum(ascii_case_insensitive)]
pub enum ProofProvider {
    #[default]
    Risc0,
    SP1,
}

impl From<ProofProvider> for HostProofProvider {
    fn from(val: ProofProvider) -> Self {
        match val {
            ProofProvider::Risc0 => HostProofProvider::Risc0,
            ProofProvider::SP1 => HostProofProvider::SP1,
        }
    }
}

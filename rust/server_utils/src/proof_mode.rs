use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Deserialize, Serialize)]
pub enum ProofMode {
    Groth16,
    Fake,
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

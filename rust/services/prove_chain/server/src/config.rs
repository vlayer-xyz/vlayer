use alloy_primitives::ChainId;
use chain::{TEST_CHAIN_ID_1, TEST_CHAIN_ID_2};
use serde::{Deserialize, Serialize};
use server_utils::ProofMode;
use std::collections::HashMap;

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct ServerConfig {
    pub rpc_urls: HashMap<ChainId, String>,
    pub port: u16,
    pub proof_mode: ProofMode,
}

impl Default for ServerConfig {
    fn default() -> Self {
        let anvil1_url = "http://localhost:8545";
        let anvil2_url = "http://localhost:8546";
        Self {
            rpc_urls: HashMap::from([
                (TEST_CHAIN_ID_1, anvil1_url.to_string()),
                (TEST_CHAIN_ID_2, anvil2_url.to_string()),
            ]),
            port: 3000,
            proof_mode: ProofMode::Groth16,
        }
    }
}

impl ServerConfig {
    pub fn new(rpc_mappings: Vec<(ChainId, String)>, proof_mode: ProofMode) -> ServerConfig {
        let default = ServerConfig::default();
        ServerConfig {
            rpc_urls: if rpc_mappings.is_empty() {
                default.rpc_urls
            } else {
                rpc_mappings.into_iter().collect()
            },
            port: default.port,
            proof_mode,
        }
    }
}

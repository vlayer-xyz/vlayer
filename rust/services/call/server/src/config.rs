use std::collections::HashMap;

use alloy_primitives::ChainId;
use chain::TEST_CHAIN_ID;
use serde::{Deserialize, Serialize};
use server_utils::ProofMode;

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct ServerConfig {
    pub rpc_urls: HashMap<ChainId, String>,
    pub host: String,
    pub port: u16,
    pub proof_mode: ProofMode,
    pub chain_proof_url: String,
}

impl Default for ServerConfig {
    fn default() -> Self {
        let anvil_url = "http://localhost:8545";
        Self {
            rpc_urls: HashMap::from([(TEST_CHAIN_ID, anvil_url.to_string())]),
            host: "127.0.0.1".into(),
            port: 3000,
            proof_mode: ProofMode::Groth16,
            chain_proof_url: String::default(),
        }
    }
}

impl ServerConfig {
    pub fn new(
        rpc_mappings: Vec<(ChainId, String)>,
        proof_mode: ProofMode,
        host: Option<String>,
        port: Option<u16>,
        chain_proof_url: String,
    ) -> ServerConfig {
        let default = ServerConfig::default();
        ServerConfig {
            rpc_urls: if rpc_mappings.is_empty() {
                default.rpc_urls
            } else {
                rpc_mappings.into_iter().collect()
            },
            host: host.unwrap_or(default.host),
            port: port.unwrap_or(default.port),
            proof_mode,
            chain_proof_url,
        }
    }
}

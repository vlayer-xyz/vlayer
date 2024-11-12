use std::{collections::HashMap, net::SocketAddr};

use alloy_primitives::ChainId;
use call_host::host::config::DEFAULT_MAX_CALLDATA_SIZE;
use chain::TEST_CHAIN_ID;
use serde::{Deserialize, Serialize};
use server_utils::ProofMode;

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct ServerConfig {
    pub rpc_urls: HashMap<ChainId, String>,
    pub socket_addr: SocketAddr,
    pub proof_mode: ProofMode,
    pub chain_proof_url: String,
    pub max_request_size: usize,
}

impl Default for ServerConfig {
    fn default() -> Self {
        let anvil_url = "http://localhost:8545";
        Self {
            rpc_urls: HashMap::from([(TEST_CHAIN_ID, anvil_url.to_string())]),
            socket_addr: "127.0.0.1:3000".parse().unwrap(),
            proof_mode: ProofMode::Groth16,
            chain_proof_url: String::default(),
            max_request_size: DEFAULT_MAX_CALLDATA_SIZE,
        }
    }
}

impl ServerConfig {
    pub fn new(
        rpc_mappings: Vec<(ChainId, String)>,
        proof_mode: ProofMode,
        host: Option<String>,
        port: Option<u16>,
        chain_proof_url: impl AsRef<str>,
    ) -> ServerConfig {
        let ServerConfig {
            mut socket_addr,
            rpc_urls,
            ..
        } = ServerConfig::default();
        if let Some(p) = port {
            socket_addr.set_port(p)
        }
        if let Some(h) = host {
            socket_addr.set_ip(h.parse().unwrap())
        };
        ServerConfig {
            rpc_urls: if rpc_mappings.is_empty() {
                rpc_urls
            } else {
                rpc_mappings.into_iter().collect()
            },
            socket_addr,
            proof_mode,
            chain_proof_url: chain_proof_url.as_ref().to_string(),
            max_request_size: DEFAULT_MAX_CALLDATA_SIZE,
        }
    }
}

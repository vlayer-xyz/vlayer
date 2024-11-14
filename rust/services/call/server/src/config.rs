use std::{collections::HashMap, net::SocketAddr};

use alloy_primitives::ChainId;
use call_host::host::config::{HostConfig, DEFAULT_MAX_CALLDATA_SIZE};
use chain::TEST_CHAIN_ID;
use common::GuestElf;
use serde::{Deserialize, Serialize};
use server_utils::ProofMode;

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct ServerConfig {
    pub rpc_urls: HashMap<ChainId, String>,
    pub socket_addr: SocketAddr,
    pub proof_mode: ProofMode,
    pub chain_proof_url: String,
    pub max_request_size: usize,
    pub verify_chain_proofs: bool,
    pub call_guest: GuestElf,
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
            verify_chain_proofs: false,
            call_guest: GuestElf::default(),
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
        verify_chain_proofs: bool,
        call_guest: GuestElf,
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
            verify_chain_proofs,
            call_guest,
        }
    }

    pub fn into_host_config(&self, start_chain_id: ChainId) -> HostConfig {
        HostConfig {
            rpc_urls: self.rpc_urls.clone(),
            start_chain_id,
            proof_mode: self.proof_mode.into(),
            chain_proof_url: self.chain_proof_url.clone(),
            max_calldata_size: self.max_request_size,
            verify_chain_proofs: self.verify_chain_proofs,
            call_guest: self.call_guest.clone(),
        }
    }
}

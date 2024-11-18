use std::{collections::HashMap, net::SocketAddr};

use alloy_primitives::ChainId;
use call_host::{Config as HostConfig, DEFAULT_MAX_CALLDATA_SIZE};
use chain::TEST_CHAIN_ID;
use common::GuestElf;
use serde::{Deserialize, Serialize};
use server_utils::ProofMode;

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Config {
    socket_addr: SocketAddr,
    rpc_urls: HashMap<ChainId, String>,
    proof_mode: ProofMode,
    chain_proof_url: String,
    max_request_size: usize,
    verify_chain_proofs: bool,
    call_guest_elf: GuestElf,
    chain_guest_elf: GuestElf,
}

impl Config {
    pub fn socket_addr(&self) -> SocketAddr {
        self.socket_addr
    }

    pub fn fake_proofs(&self) -> bool {
        matches!(self.proof_mode, ProofMode::Fake)
    }
}

pub struct ConfigBuilder {
    config: Config,
}

impl ConfigBuilder {
    pub fn new(
        chain_proof_url: impl ToString,
        call_guest_elf: GuestElf,
        chain_guest_elf: GuestElf,
    ) -> Self {
        Self {
            config: Config {
                chain_proof_url: chain_proof_url.to_string(),
                call_guest_elf,
                chain_guest_elf,
                socket_addr: "127.0.0.1:3000".parse().unwrap(),
                rpc_urls: HashMap::from([(TEST_CHAIN_ID, "http://localhost:8545".to_string())]),
                proof_mode: ProofMode::Groth16,
                max_request_size: DEFAULT_MAX_CALLDATA_SIZE,
                verify_chain_proofs: false,
            },
        }
    }

    pub fn with_rpc_mappings(
        mut self,
        mappings: impl IntoIterator<Item = (ChainId, String)>,
    ) -> Self {
        self.config.rpc_urls = mappings.into_iter().collect();
        self
    }

    pub fn with_proof_mode(mut self, proof_mode: ProofMode) -> Self {
        self.config.proof_mode = proof_mode;
        self
    }

    pub fn with_host(mut self, host: Option<String>) -> Self {
        if let Some(host) = host {
            self.config.socket_addr.set_ip(host.parse().unwrap());
        }
        self
    }

    pub fn with_port(mut self, port: Option<u16>) -> Self {
        if let Some(port) = port {
            self.config.socket_addr.set_port(port);
        }
        self
    }

    pub fn with_verify_chain_proofs(mut self, verify: bool) -> Self {
        self.config.verify_chain_proofs = verify;
        self
    }

    pub fn build(self) -> Config {
        self.config
    }
}

impl Config {
    pub fn get_host_config(&self, start_chain_id: ChainId) -> HostConfig {
        HostConfig {
            rpc_urls: self.rpc_urls.clone(),
            start_chain_id,
            proof_mode: self.proof_mode.into(),
            chain_proof_url: self.chain_proof_url.clone(),
            max_calldata_size: self.max_request_size,
            verify_chain_proofs: self.verify_chain_proofs,
            call_guest_elf: self.call_guest_elf.clone(),
            chain_guest_elf: self.chain_guest_elf.clone(),
        }
    }
}

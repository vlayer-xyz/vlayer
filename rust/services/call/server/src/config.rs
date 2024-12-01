use std::{collections::HashMap, net::SocketAddr};

use alloy_primitives::{hex::ToHexExt, ChainId};
use call_host::{Config as HostConfig, DEFAULT_MAX_CALLDATA_SIZE};
use chain::TEST_CHAIN_ID;
use common::GuestElf;
use serde::{Deserialize, Serialize};
use server_utils::ProofMode;

use crate::gas_meter::Config as GasMeterConfig;

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
    api_version: String,
    gas_meter_config: Option<GasMeterConfig>,
}

impl Config {
    pub const fn socket_addr(&self) -> SocketAddr {
        self.socket_addr
    }

    pub const fn fake_proofs(&self) -> bool {
        matches!(self.proof_mode, ProofMode::Fake)
    }

    pub fn call_guest_id(&self) -> String {
        self.call_guest_elf.id.encode_hex_with_prefix()
    }

    pub fn api_version(&self) -> String {
        self.api_version.to_string()
    }

    pub fn chain_guest_id(&self) -> String {
        self.chain_guest_elf.id.encode_hex_with_prefix()
    }

    pub fn gas_meter_config(&self) -> Option<GasMeterConfig> {
        self.gas_meter_config.clone()
    }
}

pub struct ConfigBuilder {
    config: Config,
}

impl ConfigBuilder {
    pub fn new(
        chain_proof_url: impl Into<String>,
        call_guest_elf: GuestElf,
        chain_guest_elf: GuestElf,
        api_version: String,
    ) -> Self {
        Self {
            config: Config {
                chain_proof_url: chain_proof_url.into(),
                call_guest_elf,
                chain_guest_elf,
                socket_addr: "127.0.0.1:3000".parse().unwrap(),
                rpc_urls: HashMap::from([(TEST_CHAIN_ID, "http://localhost:8545".to_string())]),
                proof_mode: ProofMode::Groth16,
                max_request_size: DEFAULT_MAX_CALLDATA_SIZE,
                verify_chain_proofs: false,
                api_version,
                gas_meter_config: None,
            },
        }
    }

    pub fn with_rpc_mappings(
        mut self,
        mappings: impl IntoIterator<Item = (ChainId, String)>,
    ) -> Self {
        self.config.rpc_urls.extend(mappings);
        self
    }

    pub const fn with_proof_mode(mut self, proof_mode: ProofMode) -> Self {
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

    pub const fn with_verify_chain_proofs(mut self, verify: bool) -> Self {
        self.config.verify_chain_proofs = verify;
        self
    }

    pub fn with_semver(mut self, semver: String) -> Self {
        self.config.api_version = semver;
        self
    }

    pub fn with_gas_meter_config(mut self, gas_meter_config: GasMeterConfig) -> Self {
        self.config.gas_meter_config = Some(gas_meter_config);
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn local_testnet_rpc_url_always_there() {
        let config =
            ConfigBuilder::new("", Default::default(), Default::default(), Default::default())
                .with_rpc_mappings(vec![])
                .build();

        assert_eq!(config.rpc_urls.get(&TEST_CHAIN_ID).unwrap(), "http://localhost:8545");
    }

    #[test]
    fn local_testnet_rpc_url_can_be_overwritten() {
        let config =
            ConfigBuilder::new("", Default::default(), Default::default(), Default::default())
                .with_rpc_mappings(vec![(TEST_CHAIN_ID, "NEW".to_string())])
                .build();

        assert_eq!(config.rpc_urls.get(&TEST_CHAIN_ID).unwrap(), "NEW");
    }

    #[test]
    fn correctly_formats_guest_id() {
        let call_elf = GuestElf::new([0; 8], &[]);
        let chain_elf = GuestElf::new([1; 8], &[]);
        let config = ConfigBuilder::new("", call_elf, chain_elf, "1.2.3".into()).build();

        assert_eq!(
            config.call_guest_id(),
            "0x0000000000000000000000000000000000000000000000000000000000000000"
        );
        assert_eq!(
            config.chain_guest_id(),
            "0x0100000001000000010000000100000001000000010000000100000001000000"
        );
        assert_eq!(config.api_version(), "1.2.3".to_string());
    }
}

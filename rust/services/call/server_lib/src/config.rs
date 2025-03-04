use std::{collections::HashMap, net::SocketAddr};

use alloy_primitives::{hex::ToHexExt, ChainId};
use call_host::Config as HostConfig;
use chain::TEST_CHAIN_ID;
use common::GuestElf;
use risc0_zkp::core::digest::Digest;
use serde::{Deserialize, Serialize};
use server_utils::ProofMode;
use strum::{Display, EnumString};

#[cfg(feature = "jwt")]
use crate::jwt::Config as JwtConfig;
use crate::{chain_proof::Config as ChainProofConfig, gas_meter::Config as GasMeterConfig};

#[derive(Clone)]
pub struct Config {
    socket_addr: SocketAddr,
    rpc_urls: HashMap<ChainId, String>,
    proof_mode: ProofMode,
    chain_proof_config: Option<ChainProofConfig>,
    max_calldata_size: usize,
    call_guest_elf: GuestElf,
    chain_guest_ids: Box<[Digest]>,
    api_version: String,
    gas_meter_config: Option<GasMeterConfig>,
    auth_mode: AuthMode,
    #[cfg(feature = "jwt")]
    jwt_config: Option<JwtConfig>,
}

#[derive(
    Debug, Display, Clone, Copy, Deserialize, Serialize, Default, PartialEq, Eq, EnumString,
)]
#[strum(ascii_case_insensitive)]
#[non_exhaustive]
pub enum AuthMode {
    #[cfg(feature = "jwt")]
    Jwt,
    #[default]
    Token,
}

impl Config {
    pub const fn auth_mode(&self) -> AuthMode {
        self.auth_mode
    }

    pub const fn socket_addr(&self) -> SocketAddr {
        self.socket_addr
    }

    pub fn rpc_urls(&self) -> HashMap<ChainId, String> {
        self.rpc_urls.clone()
    }

    pub fn chain_proof_config(&self) -> Option<ChainProofConfig> {
        self.chain_proof_config.clone()
    }

    pub fn chain_proof_url(&self) -> Option<&str> {
        self.chain_proof_config.as_ref().map(|x| x.url.as_str())
    }

    pub fn call_guest_id_hex(&self) -> String {
        self.call_guest_elf.id.encode_hex_with_prefix()
    }

    pub const fn chain_guest_id(&self) -> Digest {
        *self
            .chain_guest_ids
            .last()
            .expect("no chain guest ID provided")
    }

    pub fn chain_guest_id_hex(&self) -> String {
        self.chain_guest_id().encode_hex_with_prefix()
    }

    pub fn api_version(&self) -> String {
        self.api_version.to_string()
    }

    pub fn gas_meter_config(&self) -> Option<GasMeterConfig> {
        self.gas_meter_config.clone()
    }

    pub const fn max_calldata_size(&self) -> usize {
        self.max_calldata_size
    }

    pub const fn proof_mode(&self) -> ProofMode {
        self.proof_mode
    }

    #[cfg(feature = "jwt")]
    pub fn jwt_config(&self) -> Option<JwtConfig> {
        self.jwt_config.clone()
    }
}

pub struct ConfigBuilder {
    config: Config,
}

impl ConfigBuilder {
    pub fn new(
        call_guest_elf: GuestElf,
        chain_guest_ids: Box<[Digest]>,
        api_version: String,
    ) -> Self {
        Self {
            config: Config {
                chain_proof_config: None,
                call_guest_elf,
                chain_guest_ids,
                socket_addr: "127.0.0.1:3000".parse().unwrap(),
                rpc_urls: HashMap::from([(TEST_CHAIN_ID, "http://localhost:8545".to_string())]),
                proof_mode: ProofMode::Groth16,
                max_calldata_size: 5 * 1024 * 1024, // 5 MB
                api_version,
                gas_meter_config: None,
                auth_mode: Default::default(),
                #[cfg(feature = "jwt")]
                jwt_config: None,
            },
        }
    }

    pub const fn with_auth_mode(mut self, auth_mode: AuthMode) -> Self {
        self.config.auth_mode = auth_mode;
        self
    }

    pub fn with_chain_proof_config(
        mut self,
        chain_proof_config: impl Into<Option<ChainProofConfig>>,
    ) -> Self {
        self.config.chain_proof_config = chain_proof_config.into();
        self
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

    pub fn with_semver(mut self, semver: String) -> Self {
        self.config.api_version = semver;
        self
    }

    pub fn with_gas_meter_config(
        mut self,
        gas_meter_config: impl Into<Option<GasMeterConfig>>,
    ) -> Self {
        self.config.gas_meter_config = gas_meter_config.into();
        self
    }

    #[cfg(feature = "jwt")]
    pub fn with_jwt_config(mut self, jwt_config: impl Into<Option<JwtConfig>>) -> Self {
        self.config.jwt_config = jwt_config.into();
        self
    }

    pub fn build(self) -> Config {
        self.config
    }
}

impl From<&Config> for HostConfig {
    fn from(config: &Config) -> HostConfig {
        HostConfig {
            proof_mode: config.proof_mode.into(),
            call_guest_elf: config.call_guest_elf.clone(),
            chain_guest_ids: config.chain_guest_ids.clone(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn local_testnet_rpc_url_always_there() {
        let config = ConfigBuilder::new(Default::default(), Default::default(), Default::default())
            .with_rpc_mappings(vec![])
            .build();

        assert_eq!(config.rpc_urls.get(&TEST_CHAIN_ID).unwrap(), "http://localhost:8545");
    }

    #[test]
    fn local_testnet_rpc_url_can_be_overwritten() {
        let config = ConfigBuilder::new(Default::default(), Default::default(), Default::default())
            .with_rpc_mappings(vec![(TEST_CHAIN_ID, "NEW".to_string())])
            .build();

        assert_eq!(config.rpc_urls.get(&TEST_CHAIN_ID).unwrap(), "NEW");
    }

    #[test]
    fn correctly_formats_guest_id() {
        let call_elf = GuestElf::new([0; 8], &[]);
        let chain_guest_ids = vec![Digest::new([1; 8])].into_boxed_slice();
        let config = ConfigBuilder::new(call_elf, chain_guest_ids, "1.2.3".into()).build();

        assert_eq!(
            config.call_guest_id_hex(),
            "0x0000000000000000000000000000000000000000000000000000000000000000"
        );
        assert_eq!(
            config.chain_guest_id_hex(),
            "0x0100000001000000010000000100000001000000010000000100000001000000"
        );
        assert_eq!(config.api_version(), "1.2.3".to_string());
    }
}

use std::{collections::HashMap, net::SocketAddr as RawSocketAddr};

use alloy_primitives::{ChainId, hex::ToHexExt};
use call_host::Config as HostConfig;
use chain::TEST_CHAIN_ID;
use chain_client::ChainClientConfig;
use common::GuestElf;
use derive_more::From;
use risc0_zkp::core::digest::Digest;
use server_utils::{ProofMode, jwt::cli::Config as JwtConfig};
use thiserror::Error;

use crate::gas_meter::Config as GasMeterConfig;

#[derive(Debug, Error)]
#[error("Missing required config field: {0}")]
pub struct Error(String);

#[derive(Clone)]
pub struct Config {
    pub socket_addr: RawSocketAddr,
    pub rpc_urls: HashMap<ChainId, String>,
    pub proof_mode: ProofMode,
    pub chain_client_config: Option<ChainClientConfig>,
    pub max_calldata_size: usize,
    pub call_guest_elf: GuestElf,
    pub chain_guest_ids: Box<[Digest]>,
    pub semver: String,
    pub gas_meter_config: Option<GasMeterConfig>,
    pub jwt_config: Option<JwtConfig>,
}

impl Config {
    pub fn call_guest_id_hex(&self) -> String {
        self.call_guest_elf.id.encode_hex_with_prefix()
    }

    #[allow(clippy::expect_used)]
    pub const fn chain_guest_id(&self) -> Digest {
        *self
            .chain_guest_ids
            .last()
            .expect("no chain guest ID provided")
    }

    pub fn chain_guest_id_hex(&self) -> String {
        self.chain_guest_id().encode_hex_with_prefix()
    }
}

pub struct SocketAddr(RawSocketAddr);

impl Default for SocketAddr {
    #[allow(clippy::expect_used)]
    fn default() -> Self {
        Self(
            "127.0.0.1:3000"
                .parse()
                .expect("parsing default value 127.0.0.1:3000 should not fail"),
        )
    }
}

pub struct RpcUrls(HashMap<ChainId, String>);

impl Default for RpcUrls {
    fn default() -> Self {
        Self(HashMap::from([(TEST_CHAIN_ID, "http://localhost:8545".to_string())]))
    }
}

#[derive(From)]
pub struct MaxCalldataSize(usize);

impl Default for MaxCalldataSize {
    fn default() -> Self {
        Self(5 * 1024 * 1024) // 5 MB
    }
}

#[derive(Default)]
pub struct ConfigBuilder {
    socket_addr: SocketAddr,
    rpc_urls: RpcUrls,
    proof_mode: ProofMode,
    chain_client_config: Option<ChainClientConfig>,
    max_calldata_size: MaxCalldataSize,
    call_guest_elf: Option<GuestElf>,
    chain_guest_ids: Option<Box<[Digest]>>,
    semver: Option<String>,
    gas_meter_config: Option<GasMeterConfig>,
    jwt_config: Option<JwtConfig>,
}

impl ConfigBuilder {
    #[must_use]
    pub fn with_chain_guest_ids<T: Into<Digest>>(
        mut self,
        ids: impl IntoIterator<Item = T>,
    ) -> Self {
        self.chain_guest_ids = Some(ids.into_iter().map(Into::into).collect());
        self
    }

    #[must_use]
    pub fn with_call_guest_elf(mut self, call_guest_elf: &GuestElf) -> Self {
        self.call_guest_elf = Some(call_guest_elf.clone());
        self
    }

    #[must_use]
    pub fn with_chain_client_config(
        mut self,
        chain_client_config: impl Into<Option<ChainClientConfig>>,
    ) -> Self {
        self.chain_client_config = chain_client_config.into();
        self
    }

    #[must_use]
    pub fn with_rpc_mappings(
        mut self,
        mappings: impl IntoIterator<Item = (ChainId, String)>,
    ) -> Self {
        self.rpc_urls.0.extend(mappings);
        self
    }

    #[must_use]
    pub const fn with_proof_mode(mut self, proof_mode: ProofMode) -> Self {
        self.proof_mode = proof_mode;
        self
    }

    #[must_use]
    #[allow(clippy::unwrap_used)]
    pub fn with_host(mut self, host: Option<String>) -> Self {
        if let Some(host) = host {
            self.socket_addr.0.set_ip(host.parse().unwrap());
        }
        self
    }

    #[must_use]
    pub fn with_port(mut self, port: Option<u16>) -> Self {
        if let Some(port) = port {
            self.socket_addr.0.set_port(port);
        }
        self
    }

    #[must_use]
    pub fn with_semver(mut self, semver: impl Into<String>) -> Self {
        self.semver = Some(semver.into());
        self
    }

    #[must_use]
    pub fn with_gas_meter_config(
        mut self,
        gas_meter_config: impl Into<Option<GasMeterConfig>>,
    ) -> Self {
        self.gas_meter_config = gas_meter_config.into();
        self
    }

    #[must_use]
    pub fn with_max_calldata_size(mut self, size: usize) -> Self {
        self.max_calldata_size = size.into();
        self
    }

    #[must_use]
    pub fn with_jwt_config(mut self, jwt_config: impl Into<Option<JwtConfig>>) -> Self {
        self.jwt_config = jwt_config.into();
        self
    }

    pub fn build(self) -> Result<Config, Error> {
        let Self {
            socket_addr,
            rpc_urls,
            proof_mode,
            chain_client_config,
            max_calldata_size,
            call_guest_elf,
            chain_guest_ids,
            semver,
            gas_meter_config,
            jwt_config,
        } = self;

        let call_guest_elf = call_guest_elf.ok_or(Error("call_guest_elf".into()))?;
        let chain_guest_ids = chain_guest_ids.ok_or(Error("chain_guest_ids".into()))?;
        let semver = semver.ok_or(Error("semver".into()))?;

        Ok(Config {
            socket_addr: socket_addr.0,
            rpc_urls: rpc_urls.0,
            proof_mode,
            chain_client_config,
            max_calldata_size: max_calldata_size.0,
            call_guest_elf,
            chain_guest_ids,
            semver,
            gas_meter_config,
            jwt_config,
        })
    }
}

impl From<&Config> for HostConfig {
    fn from(config: &Config) -> HostConfig {
        HostConfig {
            proof_mode: config.proof_mode.into(),
            call_guest_elf: config.call_guest_elf.clone(),
            chain_guest_ids: config.chain_guest_ids.clone(),
            is_vlayer_test: false,
        }
    }
}

#[cfg(test)]
pub(crate) mod tests {
    use super::*;

    pub(crate) fn config_builder() -> ConfigBuilder {
        let call_elf = GuestElf::new([0; 8], &[]);
        let chain_guest_ids = vec![Digest::new([1; 8])];
        ConfigBuilder::default()
            .with_call_guest_elf(&call_elf)
            .with_chain_guest_ids(chain_guest_ids)
            .with_semver("1.2.3")
    }

    #[test]
    fn local_testnet_rpc_url_always_there() {
        let config = config_builder().with_rpc_mappings(vec![]).build().unwrap();

        assert_eq!(config.rpc_urls.get(&TEST_CHAIN_ID).unwrap(), "http://localhost:8545");
    }

    #[test]
    fn local_testnet_rpc_url_can_be_overwritten() {
        let config = config_builder()
            .with_rpc_mappings(vec![(TEST_CHAIN_ID, "NEW".to_string())])
            .build()
            .unwrap();

        assert_eq!(config.rpc_urls.get(&TEST_CHAIN_ID).unwrap(), "NEW");
    }

    #[test]
    fn correctly_formats_guest_id() {
        let config = config_builder().build().unwrap();

        assert_eq!(
            config.call_guest_id_hex(),
            "0x0000000000000000000000000000000000000000000000000000000000000000"
        );
        assert_eq!(
            config.chain_guest_id_hex(),
            "0x0100000001000000010000000100000001000000010000000100000001000000"
        );
        assert_eq!(config.semver, "1.2.3".to_string());
    }
}

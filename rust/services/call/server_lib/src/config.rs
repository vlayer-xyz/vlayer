use std::{
    collections::HashMap,
    net::{Ipv4Addr, SocketAddr as RawSocketAddr, SocketAddrV4},
    path::Path,
    str::FromStr,
    time::Duration,
};

use alloy_primitives::{ChainId, hex::ToHexExt};
use call_host::Config as HostConfig;
use chain::TEST_CHAIN_ID;
use chain_client::ChainClientConfig;
use common::GuestElf;
use common::LogFormat;
use derive_more::{Debug, From};
use guest_wrapper::{CALL_GUEST_ELF, CHAIN_GUEST_IDS};
use jwt::{Algorithm, load_jwt_key};
use risc0_zkp::core::digest::Digest;
use serde::Deserialize;
use server_utils::{ProofMode, jwt::cli::Config as JwtConfig};
use thiserror::Error;
use tracing::{info, warn};

use crate::gas_meter::Config as GasMeterConfig;

pub const CHAIN_CLIENT_POLL_INTERVAL: u64 = 5;
pub const CHAIN_CLIENT_TIMEOUT: u64 = 240;
pub const GAS_METER_TIME_TO_LIVE: u64 = 3600;

#[derive(Debug, Deserialize)]
pub struct GasMeterOptions {
    pub url: String,
    pub time_to_live: Option<u64>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum AuthOptions {
    Jwt(JwtOptions),
}

#[derive(Debug, Deserialize)]
pub struct JwtOptions {
    pub public_key: String,
    pub algorithm: String,
}

#[derive(Debug, Deserialize)]
pub struct ChainClientOptions {
    pub url: String,
    pub poll_interval: Option<u64>,
    pub timeout: Option<u64>,
}

#[derive(Debug, Deserialize)]
pub struct RpcUrl {
    pub chain_id: ChainId,
    pub host: String,
    pub port: u16,
}

#[derive(Debug, Deserialize)]
pub struct ConfigOptions {
    /// Host
    pub host: String,
    /// Port
    pub port: u16,
    /// Proof mode to use. Possible values are ["fake", "groth16"]
    pub proof_mode: ProofMode,
    /// RPC mappings
    pub rpc_urls: Vec<RpcUrl>,
    /// Chain client config
    pub chain_client: Option<ChainClientOptions>,
    /// Authentication
    pub auth: Option<AuthOptions>,
    /// Gas meter config
    pub gas_meter: Option<GasMeterOptions>,
    /// Log format
    pub log_format: LogFormat,
}

pub fn parse_config_file(path: impl AsRef<Path>) -> anyhow::Result<ConfigOptions> {
    let contents = std::fs::read_to_string(path.as_ref())?;
    let config_opts = toml::from_str(&contents)?;
    Ok(config_opts)
}

impl Default for ConfigOptions {
    fn default() -> Self {
        Self {
            host: "127.0.0.1".to_string(),
            port: 3000,
            chain_client: None,
            auth: None,
            gas_meter: None,
            proof_mode: ProofMode::default(),
            rpc_urls: Vec::default(),
            log_format: LogFormat::default(),
        }
    }
}

#[derive(Debug)]
pub struct ConfigOptionsWithVersion {
    pub semver: String,
    pub config: ConfigOptions,
}

impl TryFrom<GasMeterOptions> for GasMeterConfig {
    type Error = anyhow::Error;

    fn try_from(opts: GasMeterOptions) -> Result<Self, Self::Error> {
        let api_key = std::env::var("GAS_METER_API_KEY")?;
        let time_to_live = Duration::from_secs(opts.time_to_live.unwrap_or(GAS_METER_TIME_TO_LIVE));
        Ok(Self::new(opts.url, time_to_live, Some(api_key)))
    }
}

impl TryFrom<JwtOptions> for JwtConfig {
    type Error = anyhow::Error;

    fn try_from(opts: JwtOptions) -> Result<Self, Self::Error> {
        let algorithm = Algorithm::from_str(&opts.algorithm)?;
        let public_key = load_jwt_key(&opts.public_key, algorithm)?;
        info!(
            "Using JWT-based authorization with public key '{}' and algorithm '{}'.",
            opts.public_key, opts.algorithm
        );
        Ok(Self {
            public_key,
            algorithm,
        })
    }
}

impl From<ChainClientOptions> for ChainClientConfig {
    fn from(
        ChainClientOptions {
            url,
            poll_interval,
            timeout,
        }: ChainClientOptions,
    ) -> Self {
        let poll_interval =
            Duration::from_secs(poll_interval.unwrap_or(CHAIN_CLIENT_POLL_INTERVAL));
        let timeout = Duration::from_secs(timeout.unwrap_or(CHAIN_CLIENT_TIMEOUT));
        Self::new(url, poll_interval, timeout)
    }
}

impl TryFrom<ConfigOptionsWithVersion> for Config {
    type Error = anyhow::Error;

    fn try_from(opts: ConfigOptionsWithVersion) -> Result<Self, Self::Error> {
        if opts.config.auth.is_none() {
            warn!("Running without authorization.");
        }

        let gas_meter_config: Option<GasMeterConfig> =
            opts.config.gas_meter.map(TryInto::try_into).transpose()?;
        let jwt_config = opts
            .config
            .auth
            .map(|auth| match auth {
                AuthOptions::Jwt(jwt) => jwt.try_into(),
            })
            .transpose()?;
        let chain_client_config = opts.config.chain_client.map(Into::into);

        Ok(ConfigBuilder::default()
            .with_chain_guest_ids(CHAIN_GUEST_IDS)
            .with_call_guest_elf(&CALL_GUEST_ELF)
            .with_host(opts.config.host)
            .with_port(opts.config.port)
            .with_proof_mode(opts.config.proof_mode)
            .with_semver(opts.semver)
            .with_rpc_mappings2(opts.config.rpc_urls)
            .with_gas_meter_config(gas_meter_config)
            .with_jwt_config(jwt_config)
            .with_chain_client_config(chain_client_config)
            .build()?)
    }
}

#[derive(Debug, Error)]
#[error("Missing required config field: {0}")]
pub struct Error(String);

#[derive(Clone, Debug)]
pub struct Config {
    pub socket_addr: RawSocketAddr,
    #[debug(skip)]
    pub rpc_urls: HashMap<ChainId, String>,
    pub proof_mode: ProofMode,
    pub chain_client_config: Option<ChainClientConfig>,
    pub max_calldata_size: usize,
    #[debug(skip)]
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
    fn default() -> Self {
        Self(RawSocketAddr::V4(SocketAddrV4::new(Ipv4Addr::LOCALHOST, 3000)))
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
    pub fn with_rpc_mappings2(mut self, mappings: impl IntoIterator<Item = RpcUrl>) -> Self {
        self.rpc_urls.0.extend(mappings.into_iter().map(
            |RpcUrl {
                 chain_id,
                 host,
                 port,
             }| (chain_id, format!("{host}:{port}")),
        ));
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
    pub fn with_host(mut self, host: impl Into<String>) -> Self {
        self.socket_addr.0.set_ip(host.into().parse().unwrap());
        self
    }

    #[must_use]
    pub fn with_port(mut self, port: u16) -> Self {
        self.socket_addr.0.set_port(port);
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

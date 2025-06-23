use std::{
    collections::BTreeMap,
    marker::PhantomData,
    net::{Ipv4Addr, SocketAddr as RawSocketAddr, SocketAddrV4},
    path::{Path, PathBuf},
    str::FromStr,
    time::Duration,
};

use alloy_primitives::{ChainId, hex::ToHexExt};
use call_host::Config as HostConfig;
use chain::TEST_CHAIN_ID;
use chain_client::ChainClientConfig;
use common::{GuestElf, LogFormat};
use derive_more::{Debug, From, Into};
use guest_wrapper::{CALL_GUEST_ELF, CHAIN_GUEST_IDS};
use jwt::{Algorithm, Claim as JwtClaim, Error as JwtError, load_jwt_signing_key};
use risc0_zkp::core::digest::Digest;
use serde::{
    Deserialize, Serialize,
    de::{self, DeserializeSeed, Deserializer},
};
use server_utils::{ProofMode, jwt::config::Config as JwtConfig};
use strum::VariantNames;
use thiserror::Error;

use crate::gas_meter::{Config as GasMeterConfig, Mode as GasMeterMode};

pub const DEFAULT_HOST: &str = "127.0.0.1";
pub const DEFAULT_PORT: u16 = 3000;
pub const DEFAULT_CHAIN_CLIENT_POLL_INTERVAL: u64 = 5;
pub const DEFAULT_CHAIN_CLIENT_TIMEOUT: u64 = 240;
pub const DEFAULT_GAS_METER_TIME_TO_LIVE: u64 = 3600;

#[derive(Debug, Error)]
pub enum Error {
    #[error("Config file not found: '{}'", .0.display())]
    ConfigFile(PathBuf),
    #[error("Missing required config field: {0}")]
    ConfigField(String),
    #[error("Parsing config from toml failed: {0}")]
    ConfigToml(#[from] toml::de::Error),
    #[error("Invalid rpc url format: expected <chain-id>:<url>, got {0}")]
    RpcUrlFormat(String),
    #[error("Invalid chain id: {0}")]
    ChainId(String),
    #[error(
        "Unexpected gas-meter mode selected: '{}'. Possible values are {:?}",
        .0,
        GasMeterMode::VARIANTS
    )]
    GasMeterMode(String),
    #[error(
        "Unexpected JWT signing algorithm selected: '{}'. Possible values are {:?}",
        .0,
        Algorithm::VARIANTS
    )]
    JwtSigningAlgorithm(String),
    #[error(transparent)]
    Jwt(#[from] JwtError),
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Eq)]
pub struct GasMeterOptions {
    /// Url to the gas meter
    pub url: String,
    /// API key
    pub api_key: String,
    /// Time-to-live for gas meter requests in seconds
    pub time_to_live: Option<u64>,
    /// Mode of operation for gas meter: [bill, track]
    pub mode: String,
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum AuthOptions {
    Jwt(JwtOptions),
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Eq)]
pub struct JwtOptions {
    /// Path to the public key in PEM format
    pub public_key: String,
    /// Signing algorithm to use
    pub algorithm: String,
    /// User-defined claims
    #[serde(default, deserialize_with = "seq_string_or_struct")]
    pub claims: Vec<JwtClaim>,
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Eq)]
pub struct ChainClientOptions {
    /// Url to the chain client
    pub url: String,
    /// Poll interval in seconds
    pub poll_interval: Option<u64>,
    /// Timeout in seconds
    pub timeout: Option<u64>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Into, PartialEq, Eq)]
#[into((ChainId, String))]
pub struct RpcUrl {
    /// Chain ID
    pub chain_id: ChainId,
    /// RPC url
    pub url: String,
}

impl FromStr for RpcUrl {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let parts: Vec<&str> = s.splitn(2, ':').collect();
        if parts.len() < 2 {
            return Err(Error::RpcUrlFormat(s.to_string()));
        }
        let chain_id: ChainId = parts[0]
            .parse()
            .map_err(|_| Error::ChainId(parts[0].to_string()))?;
        let url = parts[1].to_string();
        Ok(Self { chain_id, url })
    }
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Eq)]
pub struct ConfigOptions {
    /// Host
    pub host: String,
    /// Port
    pub port: u16,
    /// Proof mode to use. Possible values are ["fake", "groth16"]
    pub proof_mode: ProofMode,
    /// RPC mappings
    #[serde(default, deserialize_with = "seq_string_or_struct")]
    pub rpc_urls: Vec<RpcUrl>,
    /// Chain client config
    pub chain_client: Option<ChainClientOptions>,
    /// Authentication
    pub auth: Option<AuthOptions>,
    /// Gas meter config
    pub gas_meter: Option<GasMeterOptions>,
    /// Log format
    pub log_format: Option<LogFormat>,
}

pub(crate) fn parse_config_file(path: impl AsRef<Path>) -> Result<ConfigOptions, Error> {
    let contents = std::fs::read_to_string(path.as_ref())
        .map_err(|_| Error::ConfigFile(path.as_ref().to_path_buf()))?;
    let config_opts: ConfigOptions = toml::from_str(&contents).map_err(Error::ConfigToml)?;
    Ok(config_opts)
}

fn seq_string_or_struct<'de, T, D, Err>(deserializer: D) -> Result<Vec<T>, D::Error>
where
    Err: Into<Error>,
    T: Deserialize<'de> + FromStr<Err = Err>,
    D: Deserializer<'de>,
{
    struct StringOrStruct<T, Err>(PhantomData<(T, Err)>);

    impl<'de, T, Err> de::Visitor<'de> for StringOrStruct<T, Err>
    where
        Err: Into<Error>,
        T: Deserialize<'de> + FromStr<Err = Err>,
    {
        type Value = T;

        fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
            formatter.write_str("string or map")
        }

        fn visit_str<E>(self, value: &str) -> Result<T, E>
        where
            E: de::Error,
        {
            FromStr::from_str(value)
                .map_err(Into::into)
                .map_err(de::Error::custom)
        }

        fn visit_map<M>(self, map: M) -> Result<T, M::Error>
        where
            M: de::MapAccess<'de>,
        {
            Deserialize::deserialize(de::value::MapAccessDeserializer::new(map))
        }
    }

    impl<'de, T, Err> DeserializeSeed<'de> for StringOrStruct<T, Err>
    where
        T: Deserialize<'de> + FromStr<Err = Err>,
        Err: Into<Error>,
    {
        type Value = T;

        fn deserialize<D>(self, deserializer: D) -> Result<Self::Value, D::Error>
        where
            D: Deserializer<'de>,
        {
            deserializer.deserialize_any(self)
        }
    }

    struct SeqStringOrStruct<T, Err>(PhantomData<(T, Err)>);

    impl<'de, T, Err> de::Visitor<'de> for SeqStringOrStruct<T, Err>
    where
        Err: Into<Error>,
        T: Deserialize<'de> + FromStr<Err = Err>,
    {
        type Value = Vec<T>;

        fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
            formatter.write_str("sequence of strings or maps")
        }

        fn visit_seq<S>(self, mut seq: S) -> Result<Self::Value, S::Error>
        where
            S: de::SeqAccess<'de>,
        {
            let mut vec = Vec::new();
            while let Some(element) = seq.next_element_seed(StringOrStruct(PhantomData))? {
                vec.push(element);
            }
            Ok(vec)
        }
    }

    deserializer.deserialize_seq(SeqStringOrStruct(PhantomData))
}

impl Default for ConfigOptions {
    fn default() -> Self {
        Self {
            host: DEFAULT_HOST.to_string(),
            port: DEFAULT_PORT,
            chain_client: None,
            auth: None,
            gas_meter: None,
            proof_mode: ProofMode::default(),
            rpc_urls: Vec::default(),
            log_format: None,
        }
    }
}

#[derive(Debug)]
pub struct ConfigOptionsWithVersion {
    pub semver: String,
    pub config: ConfigOptions,
}

impl TryFrom<GasMeterOptions> for GasMeterConfig {
    type Error = Error;

    fn try_from(
        GasMeterOptions {
            url,
            api_key,
            time_to_live,
            mode,
        }: GasMeterOptions,
    ) -> Result<Self, Self::Error> {
        let time_to_live =
            Duration::from_secs(time_to_live.unwrap_or(DEFAULT_GAS_METER_TIME_TO_LIVE));
        let mode = GasMeterMode::from_str(&mode).map_err(|_| Error::GasMeterMode(mode.clone()))?;
        Ok(Self::new(url, time_to_live, api_key, mode))
    }
}

impl TryFrom<JwtOptions> for JwtConfig {
    type Error = Error;

    fn try_from(
        JwtOptions {
            public_key,
            algorithm,
            claims,
        }: JwtOptions,
    ) -> Result<Self, Self::Error> {
        let algorithm = Algorithm::from_str(&algorithm)
            .map_err(|_| Error::JwtSigningAlgorithm(algorithm.clone()))?;
        let public_key = load_jwt_signing_key(&public_key, algorithm).map_err(Error::Jwt)?;
        Ok(Self::new(public_key, algorithm.into(), claims))
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
            Duration::from_secs(poll_interval.unwrap_or(DEFAULT_CHAIN_CLIENT_POLL_INTERVAL));
        let timeout = Duration::from_secs(timeout.unwrap_or(DEFAULT_CHAIN_CLIENT_TIMEOUT));
        Self::new(url, poll_interval, timeout)
    }
}

impl TryFrom<ConfigOptionsWithVersion> for Config {
    type Error = Error;

    fn try_from(opts: ConfigOptionsWithVersion) -> Result<Self, Self::Error> {
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

        ConfigBuilder::default()
            .with_chain_guest_ids(CHAIN_GUEST_IDS)
            .with_call_guest_elf(&CALL_GUEST_ELF)
            .with_host(opts.config.host)
            .with_port(opts.config.port)
            .with_proof_mode(opts.config.proof_mode)
            .with_semver(opts.semver)
            .with_rpc_mappings(opts.config.rpc_urls)
            .with_gas_meter_config(gas_meter_config)
            .with_jwt_config(jwt_config)
            .with_chain_client_config(chain_client_config)
            .build()
    }
}

#[derive(Clone, Debug)]
pub struct Config {
    pub socket_addr: RawSocketAddr,
    #[debug(skip)]
    pub rpc_urls: BTreeMap<ChainId, String>,
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

pub struct RpcUrls(Vec<(ChainId, String)>);

impl Default for RpcUrls {
    fn default() -> Self {
        Self(vec![(TEST_CHAIN_ID, "http://localhost:8545".to_string())])
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
        mappings: impl IntoIterator<Item = impl Into<(ChainId, String)>>,
    ) -> Self {
        self.rpc_urls.0.extend(mappings.into_iter().map(Into::into));
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

        let call_guest_elf = call_guest_elf.ok_or(Error::ConfigField("call_guest_elf".into()))?;
        let chain_guest_ids =
            chain_guest_ids.ok_or(Error::ConfigField("chain_guest_ids".into()))?;
        let semver = semver.ok_or(Error::ConfigField("semver".into()))?;
        let rpc_urls: BTreeMap<ChainId, String> = rpc_urls.0.into_iter().collect();

        Ok(Config {
            socket_addr: socket_addr.0,
            rpc_urls,
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
    use std::io::Write;

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
        let config = config_builder().build().unwrap();

        assert_eq!(config.rpc_urls.get(&TEST_CHAIN_ID).unwrap(), "http://localhost:8545");
    }

    #[test]
    fn local_testnet_rpc_url_can_be_overwritten() {
        let rpc = RpcUrl {
            chain_id: TEST_CHAIN_ID,
            url: "http://127.0.0.1:8666".to_string(),
        };
        let config = config_builder().with_rpc_mappings([rpc]).build().unwrap();

        assert_eq!(config.rpc_urls.get(&TEST_CHAIN_ID).unwrap(), "http://127.0.0.1:8666");
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

    fn save_config_file(contents: impl AsRef<str>) -> tempfile::NamedTempFile {
        let mut config_file = tempfile::NamedTempFile::new().unwrap();
        config_file
            .as_file_mut()
            .write_all(contents.as_ref().as_bytes())
            .unwrap();
        config_file
    }

    mod rpc_url_from_str {
        use super::*;

        #[test]
        fn correctly_parses_rpc_url() {
            let rpc_url_as_string = "31337:http://127.0.0.1:8545";
            assert_eq!(
                RpcUrl {
                    chain_id: 31337,
                    url: "http://127.0.0.1:8545".to_string()
                },
                RpcUrl::from_str(rpc_url_as_string).unwrap()
            );
        }

        #[test]
        fn reports_error_on_empty_string() {
            assert!(matches!(RpcUrl::from_str("").unwrap_err(), Error::RpcUrlFormat(..)));
        }

        #[test]
        fn reports_error_on_missing_chain_id() {
            assert!(matches!(RpcUrl::from_str(":some_url").unwrap_err(), Error::ChainId(..)));
        }
    }

    mod parse_config_file {
        use super::*;

        #[test]
        fn correctly_parse_config_file() {
            let config_file = save_config_file(
                r#"
        host = "127.0.0.1"
        port = 3000
        proof_mode = "groth16"

        [[rpc_urls]]
        chain_id = 31337
        url = "http://localhost:8545"

        [auth.jwt]
        public_key = "/path/to/key"
        algorithm = "rs256"

        [gas_meter]
        mode = "bill"
        url = "http://localhost:3001"
        api_key = "deadbeef"
        "#,
            );

            let opts = parse_config_file(config_file.path()).unwrap();
            assert_eq!(
                opts,
                ConfigOptions {
                    host: "127.0.0.1".to_string(),
                    port: 3000,
                    proof_mode: ProofMode::Groth16,
                    rpc_urls: vec![RpcUrl {
                        chain_id: 31337,
                        url: "http://localhost:8545".to_string()
                    }],
                    chain_client: None,
                    auth: Some(AuthOptions::Jwt(JwtOptions {
                        public_key: "/path/to/key".to_string(),
                        algorithm: "rs256".to_string(),
                        claims: Vec::new(),
                    })),
                    gas_meter: Some(GasMeterOptions {
                        url: "http://localhost:3001".to_string(),
                        api_key: "deadbeef".to_string(),
                        time_to_live: None,
                        mode: "bill".to_string(),
                    }),
                    log_format: None,
                }
            );
        }

        #[test]
        fn correctly_parse_config_file_with_alternative_rpc_urls_syntax() {
            let config_file = save_config_file(
                r#"
        host = "127.0.0.1"
        port = 3000
        proof_mode = "groth16"
        rpc_urls = ["31337:http://localhost:8545", "31338:http://localhost:8546"]
        "#,
            );

            let opts = parse_config_file(config_file.path()).unwrap();
            assert_eq!(
                opts,
                ConfigOptions {
                    host: "127.0.0.1".to_string(),
                    port: 3000,
                    proof_mode: ProofMode::Groth16,
                    rpc_urls: vec![
                        RpcUrl {
                            chain_id: 31337,
                            url: "http://localhost:8545".to_string(),
                        },
                        RpcUrl {
                            chain_id: 31338,
                            url: "http://localhost:8546".to_string(),
                        }
                    ],
                    chain_client: None,
                    auth: None,
                    gas_meter: None,
                    log_format: None,
                }
            );
        }

        #[test]
        fn reports_invalid_path_to_jwt_signing_key() {
            let config_file = save_config_file(
                r#"
                host = "0.0.0.0"
                port = 3000
                proof_mode = "fake"

                [auth.jwt]
                public_key = "/gibberish"
                algorithm = "rs256"
            "#,
            );

            let opts = parse_config_file(config_file.path()).unwrap();
            let res: Result<Config, Error> = ConfigOptionsWithVersion {
                semver: "0".to_string(),
                config: opts,
            }
            .try_into();

            assert!(matches!(res.unwrap_err(), Error::Jwt(JwtError::JwtSigningKeyNotFound(..))));
        }

        #[test]
        fn reports_invalid_jwt_signing_algorithm() {
            let config_file = save_config_file(
                r#"
                host = "0.0.0.0"
                port = 3000
                proof_mode = "fake"

                [auth.jwt]
                public_key = "docker/fixtures/jwt-authority.key.pub"
                algorithm = "ts256"
            "#,
            );

            let opts = parse_config_file(config_file.path()).unwrap();
            let res: Result<Config, Error> = ConfigOptionsWithVersion {
                semver: "0".to_string(),
                config: opts,
            }
            .try_into();

            assert!(matches!(res.unwrap_err(), Error::JwtSigningAlgorithm(..)));
        }

        #[test]
        fn correctly_parses_custom_user_jwt_claims() {
            let config_file = save_config_file(
                r#"
                host = "0.0.0.0"
                port = 3000
                proof_mode = "fake"

                [auth.jwt]
                public_key = "docker/fixtures/jwt-authority.key.pub"
                algorithm = "rs256"

                [[auth.jwt.claims]]
                name = "sub"

                [[auth.jwt.claims]]
                name = "environment"
                values = ["Test", "Production"]
            "#,
            );

            let opts = parse_config_file(config_file.path()).unwrap();
            assert_eq!(
                opts,
                ConfigOptions {
                    host: "0.0.0.0".to_string(),
                    port: 3000,
                    proof_mode: ProofMode::Fake,
                    rpc_urls: Vec::default(),
                    chain_client: None,
                    auth: Some(AuthOptions::Jwt(JwtOptions {
                        public_key: "docker/fixtures/jwt-authority.key.pub".to_string(),
                        algorithm: "rs256".to_string(),
                        claims: vec![
                            JwtClaim {
                                name: "sub".to_string(),
                                values: vec![]
                            },
                            JwtClaim {
                                name: "environment".to_string(),
                                values: vec!["Test".to_string(), "Production".to_string()]
                            }
                        ]
                    })),
                    gas_meter: None,
                    log_format: None,
                }
            );
        }
    }
}

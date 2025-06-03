mod version;

use std::str::FromStr;

use call_server_lib::{
    Config, ProofMode,
    config::{
        AuthOptions, ConfigOptions, ConfigOptionsWithVersion, JwtOptions, RpcUrl, RpcUrlOrString,
        parse_config_file,
    },
    serve,
};
use clap::Parser;
use common::{extract_rpc_url_token, init_tracing};
use config::{Config as EnvConfig, Environment};
use server_utils::set_risc0_dev_mode;
use tracing::{debug, info, warn};

#[derive(Parser)]
#[command(version = version::Version)]
struct Cli {
    /// Path to TOML config file such as config.toml.
    /// See https://book.vlayer.xyz/appendix/architecture/prover.html#toml for options.
    #[arg(long)]
    config_file: Option<String>,
}

impl TryFrom<Cli> for ConfigOptionsWithVersion {
    type Error = anyhow::Error;

    fn try_from(value: Cli) -> Result<Self, Self::Error> {
        let config = match value.config_file {
            Some(path) => parse_config_file(path)?,
            None => {
                let default_config = EnvConfig::try_from(&ConfigOptions::default())?;
                let env_config = Environment::with_prefix("VLAYER")
                    .try_parsing(true)
                    .prefix_separator("_")
                    .separator("__")
                    .list_separator(" ")
                    .with_list_parse_key("rpc_urls")
                    .ignore_empty(true);
                EnvConfig::builder()
                    .add_source(default_config)
                    .add_source(env_config)
                    .build()?
                    .try_deserialize()?
            }
        };
        let semver = version::version();
        Ok(ConfigOptionsWithVersion { semver, config })
    }
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let cli = Cli::parse();
    let opts: ConfigOptionsWithVersion = cli.try_into()?;

    let secrets: Vec<String> = opts
        .config
        .rpc_urls
        .iter()
        .filter_map(|rpc_url_or_string| {
            let res = match rpc_url_or_string {
                RpcUrlOrString::RpcUrl(rpc_url) => Some(rpc_url.clone()),
                RpcUrlOrString::String(s) => RpcUrl::from_str(s).ok(),
            };
            res.and_then(|RpcUrl { url, .. }| extract_rpc_url_token(&url))
        })
        .collect();
    init_tracing(opts.config.log_format.unwrap_or_default(), secrets);

    info!("Running vlayer serve...");

    if opts.config.proof_mode == ProofMode::Fake {
        warn!("Running in fake mode. Server will not generate real proofs.");
        set_risc0_dev_mode();
    }

    if let Some(auth) = opts.config.auth.as_ref() {
        match auth {
            AuthOptions::Jwt(JwtOptions {
                public_key,
                algorithm,
            }) => info!(
                "Using JWT-based authorization with public key '{}' and algorithm '{}'.",
                public_key, algorithm
            ),
        }
    } else {
        warn!("Running without authorization.");
    }

    let config: Config = opts.try_into()?;
    debug!("Using config: {config:#?}");

    serve(config).await?;

    Ok(())
}

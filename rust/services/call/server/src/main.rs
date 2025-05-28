mod version;

use call_server_lib::{
    Config, ProofMode,
    config::{ConfigOptions, ConfigOptionsWithVersion, RpcUrl, parse_config_file},
    serve,
};
use clap::Parser;
use common::{extract_rpc_url_token, init_tracing};
use server_utils::set_risc0_dev_mode;
use tracing::{debug, info, warn};

#[derive(Parser)]
#[command(version = version::Version)]
struct Cli {
    #[arg(long)]
    config_file: Option<String>,
}

impl TryFrom<Cli> for ConfigOptionsWithVersion {
    type Error = anyhow::Error;

    fn try_from(value: Cli) -> Result<Self, Self::Error> {
        let config = match value.config_file {
            Some(path) => parse_config_file(path)?,
            None => ConfigOptions::default(),
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
        .filter_map(|RpcUrl { host, port, .. }| extract_rpc_url_token(&format!("{host}:{port}")))
        .collect();

    init_tracing(opts.config.log_format, secrets);

    let config: Config = opts.try_into()?;
    debug!("Using config: {config:#?}");

    info!("Running vlayer serve...");
    if config.proof_mode == ProofMode::Fake {
        warn!("Running in fake mode. Server will not generate real proofs.");
        set_risc0_dev_mode();
    }

    serve(config).await?;

    Ok(())
}

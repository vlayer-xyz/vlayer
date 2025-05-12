mod version;

use std::time::Duration;

use alloy_primitives::ChainId;
use call_server_lib::{
    Config, ConfigBuilder, ProofMode, gas_meter::Config as GasMeterConfig, serve,
};
use chain_client::ChainClientConfig;
use clap::{ArgAction, Parser};
use common::{GlobalArgs, extract_rpc_url_token, init_tracing};
use guest_wrapper::{CALL_GUEST_ELF, CHAIN_GUEST_IDS};
use server_utils::{
    jwt::cli::{Args as JwtArgs, Config as JwtConfig},
    set_risc0_dev_mode,
};
use tracing::{info, warn};

#[derive(Parser)]
#[command(version = version::Version)]
struct Cli {
    #[arg(long, action = ArgAction::Append, value_parser = parse_rpc_url)]
    rpc_url: Vec<(ChainId, String)>,

    #[arg(long, value_enum)]
    proof: Option<ProofMode>,

    #[arg(long, default_value = "127.0.0.1")]
    host: Option<String>,

    #[arg(long, short, default_value = "3000")]
    port: Option<u16>,

    #[arg(long, group = "chain_proof")]
    chain_proof_url: Option<String>,

    #[arg(long, requires = "chain_proof", value_parser = humantime::parse_duration, default_value = "5s")]
    chain_proof_poll_interval: Option<Duration>,

    #[arg(long, requires = "chain_proof", value_parser = humantime::parse_duration, default_value = "240s")]
    chain_proof_timeout: Option<Duration>,

    #[clap(flatten)]
    jwt_args: JwtArgs,

    #[arg(
        long,
        group = "gas_meter",
        requires_all = ["auth", "gas_meter_api_key"],
        env
    )]
    gas_meter_url: Option<String>,

    #[arg(long, requires = "gas_meter", value_parser = humantime::parse_duration, default_value = "1h")]
    gas_meter_ttl: Option<Duration>,

    #[arg(long, requires = "gas_meter", env)]
    gas_meter_api_key: Option<String>,

    #[clap(flatten)]
    global_args: GlobalArgs,
}

impl Cli {
    fn into_config(self, api_version: String) -> anyhow::Result<Config> {
        let proof_mode = self.proof.unwrap_or_default();
        let gas_meter_config = self
            .gas_meter_url
            .zip(Some(self.gas_meter_ttl.unwrap_or_default()))
            .map(|(url, ttl)| GasMeterConfig::new(url, ttl, self.gas_meter_api_key));
        let chain_proof_config = self
            .chain_proof_url
            .zip(Some((
                self.chain_proof_poll_interval.unwrap_or_default(),
                self.chain_proof_timeout.unwrap_or_default(),
            )))
            .map(|(url, (poll_interval, timeout))| {
                ChainClientConfig::new(url, poll_interval, timeout)
            });
        let jwt_config: Option<JwtConfig> = self.jwt_args.try_into()?;
        Ok(ConfigBuilder::default()
            .with_call_guest_elf(&CALL_GUEST_ELF)
            .with_chain_guest_ids(CHAIN_GUEST_IDS)
            .with_semver(api_version)
            .with_chain_client_config(chain_proof_config)
            .with_gas_meter_config(gas_meter_config)
            .with_rpc_mappings(self.rpc_url)
            .with_proof_mode(proof_mode)
            .with_host(self.host)
            .with_port(self.port)
            .with_jwt_config(jwt_config)
            .build()?)
    }
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let api_version = version::version();
    let cli = Cli::parse();

    let secrets: Vec<String> = cli
        .rpc_url
        .iter()
        .filter_map(|(_chain_id, url)| extract_rpc_url_token(url))
        .collect();

    init_tracing(cli.global_args.log_format, secrets);

    let config = cli.into_config(api_version)?;

    info!("Running vlayer serve...");
    if config.proof_mode == ProofMode::Fake {
        warn!("Running in fake mode. Server will not generate real proofs.");
        set_risc0_dev_mode();
    }

    serve(config).await?;

    Ok(())
}

fn parse_rpc_url(s: &str) -> Result<(ChainId, String), String> {
    let parts: Vec<&str> = s.split(':').collect();
    if parts.len() < 2 {
        return Err("expected <chain-id>:<url>".to_string());
    }
    let chain_id: ChainId = parts[0]
        .parse()
        .map_err(|_| format!("Invalid chain ID: {}", parts[0]))?;
    let url = parts[1..].join(":");
    Ok((chain_id, url))
}

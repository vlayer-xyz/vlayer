mod version;

use std::time::Duration;

use alloy_primitives::ChainId;
#[cfg(feature = "jwt")]
use call_server_lib::jwt::Algorithm;
use call_server_lib::{
    chain_proof::Config as ChainProofConfig, gas_meter::Config as GasMeterConfig, serve, Config,
    ConfigBuilder, ProofMode,
};
use clap::{ArgAction, Parser};
use common::{init_tracing, GlobalArgs, LogFormat};
use guest_wrapper::{CALL_GUEST_ELF, CHAIN_GUEST_IDS};
use server_utils::set_risc0_dev_mode;
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

    #[arg(long, requires = "chain_proof", value_parser = humantime::parse_duration, default_value = "180s")]
    chain_proof_timeout: Option<Duration>,

    #[arg(long, group = "gas_meter", env)]
    gas_meter_url: Option<String>,

    #[arg(long, requires = "gas_meter", value_parser = humantime::parse_duration, default_value = "1h")]
    gas_meter_ttl: Option<Duration>,

    #[arg(long, requires = "gas_meter", env)]
    gas_meter_api_key: Option<String>,

    #[cfg(feature = "jwt")]
    #[arg(long, group = "jwt")]
    public_key: std::path::PathBuf,

    #[cfg(feature = "jwt")]
    #[arg(long, group = "jwt", default_value = "RS256")]
    algorithm: Option<Algorithm>,

    #[clap(flatten)]
    global_args: GlobalArgs,
}

impl Cli {
    #[allow(unused_mut)]
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
                ChainProofConfig::new(url, poll_interval, timeout)
            });
        let mut builder = ConfigBuilder::default()
            .with_call_guest_elf(&CALL_GUEST_ELF)
            .with_chain_guest_ids(CHAIN_GUEST_IDS)
            .with_semver(api_version)
            .with_chain_proof_config(chain_proof_config)
            .with_gas_meter_config(gas_meter_config)
            .with_rpc_mappings(self.rpc_url)
            .with_proof_mode(proof_mode)
            .with_host(self.host)
            .with_port(self.port);

        #[cfg(feature = "jwt")]
        {
            builder =
                with_jwt_config(builder, self.public_key, self.algorithm.unwrap_or_default())?;
        }

        Ok(builder.build()?)
    }
}

#[cfg(feature = "jwt")]
fn with_jwt_config(
    mut builder: ConfigBuilder,
    public_key: impl AsRef<std::path::Path>,
    alg: Algorithm,
) -> anyhow::Result<ConfigBuilder> {
    use anyhow::Context;
    use call_server_lib::jwt::{Config as JwtConfig, DecodingKey};
    let public_key = std::fs::read_to_string(public_key.as_ref()).with_context(|| {
        format!("Failed to open file '{}' for reading", public_key.as_ref().display())
    })?;
    let key = DecodingKey::from_rsa_pem(public_key.as_bytes())?;
    builder = builder.with_jwt_config(JwtConfig::new(key, alg));
    Ok(builder)
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let api_version = version::version();
    let cli = Cli::parse();

    init_tracing(cli.global_args.log_format.unwrap_or(LogFormat::Plain));

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

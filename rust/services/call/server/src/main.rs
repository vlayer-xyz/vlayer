mod version;

use alloy_primitives::ChainId;
use call_guest_wrapper::GUEST_ELF as CALL_GUEST_ELF;
use call_server_lib::{gas_meter::Config as GasMeterConfig, serve, Config, ProofMode};
use chain_guest_wrapper::GUEST_ELF as CHAIN_GUEST_ELF;
use clap::{ArgAction, Parser, ValueEnum};
use common::{init_tracing, GlobalArgs, LogFormat};
use tracing::{info, warn};

#[derive(Parser)]
#[command(version = version::Version)]
struct Cli {
    #[arg(long, action = ArgAction::Append, value_parser = parse_rpc_url)]
    rpc_url: Vec<(ChainId, String)>,

    #[arg(long, value_enum)]
    proof: Option<ProofModeArg>,

    #[arg(long, default_value = "127.0.0.1")]
    host: Option<String>,

    #[arg(long, short, default_value = "3000")]
    port: Option<u16>,

    #[arg(long)]
    chain_proof_url: Option<String>,

    #[arg(long, group = "gas_meter", env)]
    gas_meter_url: Option<String>,

    #[arg(long, requires = "gas_meter", default_value = "3600")]
    gas_meter_ttl: Option<u64>,

    #[arg(long, requires = "gas_meter", env)]
    gas_meter_api_key: Option<String>,

    #[clap(flatten)]
    global_args: GlobalArgs,
}

impl Cli {
    fn into_config(self, api_version: String) -> Config {
        let proof_mode = self.proof.unwrap_or_default().map();
        let gas_meter_config = self
            .gas_meter_url
            .zip(Some(self.gas_meter_ttl.unwrap_or_default()))
            .map(|(url, ttl)| GasMeterConfig::new(url, ttl, self.gas_meter_api_key));
        call_server_lib::ConfigBuilder::new(
            CALL_GUEST_ELF.clone(),
            CHAIN_GUEST_ELF.clone(),
            api_version,
        )
        .with_chain_proof_url(self.chain_proof_url)
        .with_gas_meter_config(gas_meter_config)
        .with_rpc_mappings(self.rpc_url)
        .with_proof_mode(proof_mode)
        .with_host(self.host)
        .with_port(self.port)
        .build()
    }
}

#[derive(Clone, Debug, ValueEnum, Default, PartialEq, Eq)]
enum ProofModeArg {
    #[default]
    Fake,
    Groth16,
}

impl ProofModeArg {
    const fn map(self) -> ProofMode {
        match self {
            ProofModeArg::Groth16 => ProofMode::Groth16,
            ProofModeArg::Fake => ProofMode::Fake,
        }
    }
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let api_version = version::version();
    let cli = Cli::parse();

    init_tracing(cli.global_args.log_format.unwrap_or(LogFormat::Plain));

    let config = cli.into_config(api_version);

    info!("Running vlayer serve...");
    if config.fake_proofs() {
        warn!("Running in fake mode. Server will not generate real proofs.");
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

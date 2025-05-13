use std::{path::PathBuf, sync::Arc, time::Duration};

use alloy_primitives::ChainId;
use chain_host::{
    AppendStrategy, Host, HostConfig, PrependStrategy, ProofMode, set_risc0_dev_mode,
};
use clap::Parser;
use common::{GlobalArgs, extract_rpc_url_token, init_tracing};
use dotenvy::dotenv;
use ethers::{providers::Http, types::BlockNumber as BlockTag};
use guest_wrapper::{CHAIN_GUEST_ELF, CHAIN_GUEST_IDS};
use retry::HostErrorFilter;
use risc0_zkp::core::digest::Digest;
use strum::{Display, EnumString};
use tokio::sync::Mutex;
use tower::{Service, ServiceBuilder, retry::budget::TpsBudget};
use tracing::error;
use version::version;

mod retry;

const DEPOSIT_TIME_TO_LIVE: Duration = Duration::from_secs(60);
const MIN_RETRIES_PER_SECOND: u32 = 3;
const RETRY_PERCENT: f32 = 0.01;

#[derive(Debug, Default, Copy, Clone, PartialEq, Eq, EnumString, Display)]
#[strum(ascii_case_insensitive, serialize_all = "kebab-case")]
enum Mode {
    /// Init database if empty and run append-prepend in a loop
    #[default]
    Continuous,
    /// Init database and exit
    Init,
    /// Perform single append-prepend and exit
    AppendPrepend,
}

#[derive(Parser)]
#[command(version = version())]
struct Cli {
    #[arg(long, env, help = "Blockchain RPC URL")]
    rpc_url: String,

    #[arg(long, env, help = "ID of the chain to index", default_value_t = 1)]
    chain_id: ChainId,

    #[arg(long, env, help = "Proof generation mode", default_value_t = ProofMode::Fake)]
    proof_mode: ProofMode,

    #[arg(long, env, help = "Operation mode", default_value_t = Mode::Continuous)]
    mode: Mode,

    #[arg(
        long,
        env,
        help = "Path to chain database directory",
        default_value = "chain_db"
    )]
    db_path: PathBuf,

    #[arg(
        long,
        env,
        help = "Block from which synchronization will start if database is empty",
        default_value_t = BlockTag::Latest
    )]
    start_block: BlockTag,

    #[arg(
        long,
        env,
        help = "Maximum number of historical blocks prepended in a single batch"
    )]
    max_back_propagation_blocks: u64,

    #[arg(
        long,
        env,
        help = "Maximum number of new blocks appended in a single batch"
    )]
    max_head_blocks: u64,

    #[arg(
        long,
        env,
        help = "Minimum number of confirmations required for a block to be appended"
    )]
    confirmations: u64,

    #[clap(flatten)]
    global_args: GlobalArgs,
}

impl From<Cli> for HostConfig {
    fn from(cli: Cli) -> Self {
        HostConfig {
            rpc_url: cli.rpc_url,
            chain_id: cli.chain_id,
            proof_mode: cli.proof_mode,
            db_path: cli.db_path,
            elf: CHAIN_GUEST_ELF.clone(),
            chain_guest_ids: CHAIN_GUEST_IDS
                .iter()
                .map(|bytes| Digest::from_bytes(*bytes))
                .collect(),
            start_block: cli.start_block,
            prepend_strategy: PrependStrategy::new(cli.max_back_propagation_blocks),
            append_strategy: AppendStrategy::new(cli.max_head_blocks, cli.confirmations),
        }
    }
}

async fn run_continuous(host: Host<Http>) -> anyhow::Result<()> {
    let host = Arc::new(Mutex::new(host));
    let budget = TpsBudget::new(DEPOSIT_TIME_TO_LIVE, MIN_RETRIES_PER_SECOND, RETRY_PERCENT);
    let mut host_service = ServiceBuilder::new()
        .retry(retry::Policy::<HostErrorFilter>::new(budget))
        .timeout(Duration::from_secs(60))
        .service_fn(|_| {
            let host = host.clone();
            async move { host.lock().await.poll_commit().await }
        });
    loop {
        host_service
            .call(())
            .await
            .map_err(|e| anyhow::anyhow!(e))?;
    }
}

async fn run(mode: Mode, config: HostConfig) -> anyhow::Result<()> {
    let mut host = Host::try_new(config)?;

    match mode {
        Mode::Init => {
            let chain_update = host.initialize().await?;
            host.commit(chain_update)?;
            Ok(())
        }
        Mode::AppendPrepend => {
            let chain_update = host
                .append_prepend()
                .await?
                .ok_or(anyhow::anyhow!("No chain update to apply"))?;
            host.commit(chain_update)?;
            Ok(())
        }
        Mode::Continuous => run_continuous(host).await,
    }
}

#[tokio::main]
async fn main() {
    dotenv().ok();
    let cli = Cli::parse();

    let secrets: Vec<String> = match extract_rpc_url_token(&cli.rpc_url) {
        Some(token) => vec![token],
        None => vec![],
    };
    init_tracing(cli.global_args.log_format, secrets);

    let mode = cli.mode;
    let config: HostConfig = cli.into();

    if config.proof_mode == ProofMode::Groth16 {
        error!(
            "Groth16 proof mode is not supported by chain workers as they need to compose proofs. Use succinct or fake"
        );
        std::process::exit(1);
    }

    if config.proof_mode == ProofMode::Fake {
        set_risc0_dev_mode();
    }

    if let Err(e) = run(mode, config).await {
        error!("{}", e.to_string());
        std::process::exit(1)
    }
}

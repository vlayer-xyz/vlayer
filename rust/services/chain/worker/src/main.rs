use std::{path::PathBuf, sync::Arc, time::Duration};

use alloy_primitives::ChainId;
use chain_guest_wrapper::GUEST_ELF;
use chain_host::{AppendStrategy, Host, HostConfig, PrependStrategy, ProofMode};
use clap::Parser;
use dotenvy::dotenv;
use ethers::types::BlockNumber as BlockTag;
use retry::HostErrorFilter;
use tokio::sync::Mutex;
use tower::{retry::budget::TpsBudget, Service, ServiceBuilder};
use trace::init_tracing;

mod retry;
mod trace;

const DEPOSIT_TIME_TO_LIVE: Duration = Duration::from_secs(60);
const MIN_RETRIES_PER_SECOND: u32 = 3;
const RETRY_PERCENT: f32 = 0.01;

#[derive(Parser)]
#[command(version)]
struct Cli {
    #[arg(long, env, help = "Blockchain RPC URL")]
    rpc_url: String,

    #[arg(long, env, help = "ID of the chain to index", default_value_t = 1)]
    chain_id: ChainId,

    #[arg(long, env, help = "Proof generation mode", default_value_t = ProofMode::Fake)]
    proof_mode: ProofMode,

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
}

impl From<Cli> for HostConfig {
    fn from(cli: Cli) -> Self {
        HostConfig {
            rpc_url: cli.rpc_url,
            chain_id: cli.chain_id,
            proof_mode: cli.proof_mode,
            db_path: cli.db_path,
            elf: GUEST_ELF.clone(),
            start_block: cli.start_block,
            prepend_strategy: PrependStrategy::new(cli.max_back_propagation_blocks),
            append_strategy: AppendStrategy::new(cli.max_head_blocks, cli.confirmations),
        }
    }
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    dotenv().ok();
    init_tracing();

    let config = Cli::parse().into();

    let host = Arc::new(Mutex::new(Host::try_new(config)?));
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

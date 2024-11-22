use std::path::PathBuf;

use alloy_primitives::ChainId;
use chain_host::{AppendStrategy, Host, HostConfig, PrependStrategy, ProofMode};
use clap::Parser;
use dotenvy::dotenv;
use trace::init_tracing;

mod trace;

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

    let mut host = Host::try_new(config)?;
    loop {
        host.poll_commit().await?;
    }
}

use std::{net::SocketAddr, path::PathBuf};

use chain_db::{ChainDb, Mode};
use chain_server_lib::{ServerConfig, serve};
use clap::Parser;
use common::{GlobalArgs, init_tracing};
use dotenvy::dotenv;
use guest_wrapper::CHAIN_GUEST_IDS;
use risc0_zkp::core::digest::Digest;
use tracing::error;
use version::version;

#[derive(Parser)]
#[command(version = version())]
struct Cli {
    #[arg(
        long,
        short,
        env,
        help = "Socket address to listen on",
        default_value = "0.0.0.0:3001"
    )]
    listen_addr: SocketAddr,

    #[arg(
        long,
        short,
        env,
        help = "Path to chain database directory",
        default_value = "chain_db"
    )]
    db_path: PathBuf,

    #[clap(flatten)]
    global_args: GlobalArgs,
}

#[tokio::main]
async fn main() {
    if let Err(e) = run().await {
        error!("{}", e.to_string());
        std::process::exit(1)
    }
}

async fn run() -> anyhow::Result<()> {
    dotenv().ok();
    let cli = Cli::parse();
    init_tracing(cli.global_args.log_format, vec![]);

    let config = ServerConfig::new(cli.listen_addr);
    let db = ChainDb::mdbx(
        cli.db_path,
        Mode::ReadOnly,
        CHAIN_GUEST_IDS.into_iter().map(Digest::from_bytes),
    )?;

    serve(config, db).await?;

    Ok(())
}

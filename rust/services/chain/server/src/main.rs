mod config;
mod error;
mod handlers;
mod server;
mod trace;

use std::{net::SocketAddr, path::PathBuf};

use chain_db::{ChainDb, Mode};
use chain_guest_wrapper::GUEST_ELF;
use clap::Parser;
use config::ServerConfig;
use dotenvy::dotenv;
use server::server;
use tokio::net::TcpListener;
use trace::init_tracing;
use tracing::info;

pub async fn serve(config: ServerConfig, db: ChainDb) -> anyhow::Result<()> {
    let listener = TcpListener::bind(config.listen_addr).await?;

    info!("Listening on {}", listener.local_addr()?);
    axum::serve(listener, server(db)).await?;

    Ok(())
}

#[derive(Parser)]
#[command(version)]
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
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    dotenv().ok();
    init_tracing();

    let cli = Cli::parse();
    let config = ServerConfig::new(cli.listen_addr);
    let db = ChainDb::mdbx(cli.db_path, Mode::ReadOnly, GUEST_ELF.clone())?;

    serve(config, db).await?;

    Ok(())
}

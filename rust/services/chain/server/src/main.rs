use std::{net::SocketAddr, path::PathBuf};

use chain_db::{ChainDb, Mode};
use chain_guest_wrapper::GUEST_ELF;
use chain_server_lib::{init_tracing, serve, ServerConfig};
use clap::Parser;
use dotenvy::dotenv;

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

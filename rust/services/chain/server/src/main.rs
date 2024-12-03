use std::{net::SocketAddr, path::PathBuf};

use chain_db::{ChainDb, Mode};
use chain_guest_wrapper::GUEST_ELF;
use chain_server_lib::{init_tracing, serve, ServerConfig};
use clap::{Parser, ValueEnum};
use dotenvy::dotenv;

#[derive(Clone, Debug, ValueEnum, Default, PartialEq, Eq)]
enum LogFormatArg {
    #[default]
    Plain,
    Json,
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

    /// A format for printing logs.
    #[arg(
        long,
        global = true,
        value_enum,
        env = "VLAYER_LOG_FORMAT",
        default_value = "plain"
    )]
    log_format: Option<LogFormatArg>,
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    dotenv().ok();
    let cli = Cli::parse();
    init_tracing(cli.log_format == Some(LogFormatArg::Json));

    let config = ServerConfig::new(cli.listen_addr);
    let db = ChainDb::mdbx(cli.db_path, Mode::ReadOnly, GUEST_ELF.clone())?;

    serve(config, db).await?;

    Ok(())
}

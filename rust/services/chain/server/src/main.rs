use std::{net::SocketAddr, path::PathBuf};

use chain_db::{ChainDb, Mode};
use chain_guest_wrapper::GUEST_ELF;
use chain_server_lib::{serve, ServerConfig};
use clap::Parser;
use common::{init_tracing, GlobalArgs, LogFormat};
use dotenvy::dotenv;
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
async fn main() -> anyhow::Result<()> {
    dotenv().ok();
    let cli = Cli::parse();
    init_tracing(cli.global_args.log_format.unwrap_or(LogFormat::Plain));

    let config = ServerConfig::new(cli.listen_addr);
    let db = ChainDb::mdbx(cli.db_path, Mode::ReadOnly, GUEST_ELF.clone())?;

    serve(config, db).await?;

    Ok(())
}

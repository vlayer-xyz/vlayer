mod config;
mod server;

use std::{net::SocketAddr, path::PathBuf};

use clap::Parser;
use common::{init_tracing, GlobalArgs, LogFormat};
use config::Config;
use dotenvy::dotenv;
use server::serve;

#[derive(Debug, clap::Args)]
#[group(required = false, multiple = false)]
struct PrivateKeyArgs {
    #[arg(
        long,
        short = 'k',
        env = "PRIVATE_KEY",
        help = "Private key in PEM format"
    )]
    private_key: Option<String>,
    #[arg(long, short = 'f', env = "PRIVATE_KEY_PATH", help = "Path to PEM file")]
    private_key_path: Option<PathBuf>,
}

#[derive(Debug, Parser)]
#[command(version)]
struct Cli {
    #[arg(
        long,
        short,
        env,
        help = "Socket address to listen on",
        default_value = "127.0.0.1:3002"
    )]
    listen_addr: SocketAddr,

    #[clap(flatten)]
    private_key: PrivateKeyArgs,

    #[clap(flatten)]
    global_args: GlobalArgs,
}

impl PrivateKeyArgs {
    fn private_key(self) -> Result<Option<String>, std::io::Error> {
        self.private_key_path
            .map(std::fs::read_to_string)
            .transpose()
            .map(|key| key.or(self.private_key))
    }
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    dotenv().ok();
    let cli = Cli::parse();
    init_tracing(cli.global_args.log_format.unwrap_or(LogFormat::Plain));

    let config = Config::new(cli.listen_addr, cli.private_key.private_key()?);

    serve(config).await?;

    Ok(())
}

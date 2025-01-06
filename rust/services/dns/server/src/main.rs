mod config;
mod server;
mod verifiable_dns;

use std::net::SocketAddr;

use clap::Parser;
use common::{init_tracing, GlobalArgs, LogFormat};
use config::Config;
use dotenvy::dotenv;
use server::serve;

#[derive(Parser)]
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
    global_args: GlobalArgs,
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    dotenv().ok();
    let cli = Cli::parse();
    init_tracing(cli.global_args.log_format.unwrap_or(LogFormat::Plain));

    let config = Config::new(cli.listen_addr);

    serve(config).await?;

    Ok(())
}

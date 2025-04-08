mod cli;
mod config;
mod server;

use clap::Parser;
use common::{LogFormat, init_tracing};
use config::Config;
use dotenvy::dotenv;
use server::serve;

use crate::cli::Cli;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    dotenv().ok();
    let cli = Cli::parse();
    init_tracing(cli.global_args.log_format.unwrap_or(LogFormat::Plain));

    let config = Config::new(cli.listen_addr, cli.private_key.private_key()?);

    serve(config).await?;

    Ok(())
}

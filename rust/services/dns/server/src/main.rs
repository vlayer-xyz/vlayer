mod cli;
mod config;
mod server;

use clap::Parser;
use common::init_tracing;
use config::ConfigBuilder;
use dotenvy::dotenv;
use server::serve;
use server_utils::jwt::cli::Config as JwtConfig;

use crate::cli::Cli;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    dotenv().ok();
    let cli = Cli::parse();
    init_tracing(cli.global_args.log_format, vec![]);

    let jwt_config: Option<JwtConfig> = cli.jwt_args.try_into()?;
    let config = ConfigBuilder::default()
        .with_socket_addr(cli.listen_addr)
        .with_private_key(cli.private_key.private_key()?)
        .with_jwt_config(jwt_config)
        .build();

    serve(config).await?;

    Ok(())
}

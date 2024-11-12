mod config;
mod error;
mod handlers;
mod server;
mod trace;

use std::env::var;

use chain_db::{ChainDb, Mode};
use config::ServerConfig;
use dotenvy::dotenv;
use server::server;
use trace::init_tracing;
use tracing::info;

pub async fn serve(config: ServerConfig, db: ChainDb) -> anyhow::Result<()> {
    let listener = tokio::net::TcpListener::bind(config.socket_addr).await?;

    info!("Listening on {}", listener.local_addr()?);
    axum::serve(listener, server(db)).await?;

    Ok(())
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    dotenv().ok();
    init_tracing();

    let config = ServerConfig::default();
    let db = ChainDb::mdbx(var("DB_PATH")?, Mode::ReadOnly)?;

    serve(config, db).await?;

    Ok(())
}

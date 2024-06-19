use app::app;
use trace::init_tracing;
use tracing::info;

mod app;
mod handlers;
mod trace;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    init_tracing();

    let listener = tokio::net::TcpListener::bind("127.0.0.1:3000").await?;

    info!("listening on {}", listener.local_addr()?);
    Ok(axum::serve(listener, app()).await?)
}

use server::server;
use trace::init_tracing;
use tracing::info;

mod error;
mod handlers;
mod json_rpc;
mod layers;
mod server;
#[cfg(test)]
mod test_helpers;
mod trace;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    init_tracing()?;

    let listener = tokio::net::TcpListener::bind("127.0.0.1:3000").await?;

    info!("listening on {}", listener.local_addr()?);
    axum::serve(listener, server()).await?;

    opentelemetry::global::shutdown_tracer_provider();

    Ok(())
}

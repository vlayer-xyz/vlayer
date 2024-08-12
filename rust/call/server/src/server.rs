use std::sync::Arc;

use crate::{config::ServerConfig, handlers::v_call::VCall};
use axum::{routing::post, Router};
use server_utils::{init_trace_layer, route, RequestIdLayer};
use tracing::info;

pub async fn serve(config: ServerConfig) -> anyhow::Result<()> {
    let listener =
        tokio::net::TcpListener::bind(format!("{}:{}", "127.0.0.1", config.port)).await?;

    info!("listening on {}", listener.local_addr()?);
    axum::serve(listener, server(config)).await?;

    Ok(())
}

pub fn server(config: ServerConfig) -> Router {
    config.proof_mode.set_risc0_flag();
    let config = Arc::new(config);

    Router::new()
        .route(
            "/",
            post(move |req| route(req, "v_call", VCall::new(config.clone()))),
        )
        .layer(init_trace_layer())
        // NOTE: RequestIdLayer should be added after the Trace layer
        .layer(RequestIdLayer)
}

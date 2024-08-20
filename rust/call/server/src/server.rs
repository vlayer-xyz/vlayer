use std::{pin::Pin, sync::Arc};

use crate::{config::ServerConfig, handlers::v_call::v_call};
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
    let jrpc_handler = move |params| Box::pin(v_call(config.clone(), params)) as Pin<Box<_>>;
    let http_handler = |req| async move { route(req, "v_call", jrpc_handler).await };

    Router::new()
        .route("/", post(http_handler))
        .layer(init_trace_layer())
        // NOTE: RequestIdLayer should be added after the Trace layer
        .layer(RequestIdLayer)
}

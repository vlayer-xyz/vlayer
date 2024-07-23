use std::sync::Arc;

use crate::json_rpc::json_rpc;
use crate::layers::request_id::RequestIdLayer;
use crate::layers::trace::init_trace_layer;
use axum::{routing::post, Router};
use serde::{Deserialize, Serialize};
use tracing::info;

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Config {
    pub url: String,
    pub port: u16,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            url: "http://localhost:8545".to_string(),
            port: 3000,
        }
    }
}

pub async fn serve(config: Config) -> anyhow::Result<()> {
    let listener =
        tokio::net::TcpListener::bind(format!("{}:{}", "127.0.0.1", config.port)).await?;

    info!("listening on {}", listener.local_addr()?);
    axum::serve(listener, server(config)).await?;

    Ok(())
}

pub fn server(config: Config) -> Router {
    let config = Arc::new(config);
    Router::new()
        .route("/", post(move |req| json_rpc(config, req)))
        .layer(init_trace_layer())
        // NOTE: RequestIdLayer should be added after the Trace layer
        .layer(RequestIdLayer)
}

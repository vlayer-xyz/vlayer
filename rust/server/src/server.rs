use std::sync::Arc;

use crate::json_rpc::json_rpc;
use crate::layers::request_id::RequestIdLayer;
use crate::layers::trace::init_trace_layer;
use alloy_primitives::ChainId;
use axum::{routing::post, Router};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use tracing::info;
use vlayer_engine::config::{MAINNET_ID, SEPOLIA_ID};

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct ServerConfig {
    pub rpc_urls: HashMap<ChainId, String>,
    pub port: u16,
}

impl Default for ServerConfig {
    fn default() -> Self {
        let anvil_url = "http://localhost:8545";
        Self {
            rpc_urls: HashMap::from([
                (SEPOLIA_ID, anvil_url.to_string()),
                (MAINNET_ID, anvil_url.to_string()),
            ]),
            port: 3000,
        }
    }
}

impl ServerConfig {
    pub fn new(rpc_mappings: Vec<(ChainId, String)>) -> ServerConfig {
        if rpc_mappings.is_empty() {
            ServerConfig::default()
        } else {
            ServerConfig {
                rpc_urls: rpc_mappings.into_iter().collect(),
                port: 3000,
            }
        }
    }
}

pub async fn serve(config: ServerConfig) -> anyhow::Result<()> {
    let listener =
        tokio::net::TcpListener::bind(format!("{}:{}", "127.0.0.1", config.port)).await?;

    info!("listening on {}", listener.local_addr()?);
    axum::serve(listener, server(config)).await?;

    Ok(())
}

pub fn server(config: ServerConfig) -> Router {
    let config = Arc::new(config);
    Router::new()
        .route("/", post(move |req| json_rpc(config, req)))
        .layer(init_trace_layer())
        // NOTE: RequestIdLayer should be added after the Trace layer
        .layer(RequestIdLayer)
}

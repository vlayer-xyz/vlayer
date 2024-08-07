use std::sync::Arc;

use crate::json_rpc::json_rpc;
use alloy_primitives::ChainId;
use axum::{routing::post, Router};
use call_engine::config::{MAINNET_ID, SEPOLIA_ID};
use serde::{Deserialize, Serialize};
use server_utils::{init_trace_layer, RequestIdLayer};
use std::collections::HashMap;
use tracing::info;

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct ServerConfig {
    pub rpc_urls: HashMap<ChainId, String>,
    pub port: u16,
    pub proof_mode: ProofMode,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub enum ProofMode {
    Groth16,
    Fake,
}

impl ProofMode {
    fn set_risc0_flag(&self) {
        let value = match self {
            ProofMode::Groth16 => "0",
            ProofMode::Fake => "1",
        };
        std::env::set_var("RISC0_DEV_MODE", value);
    }
}

impl Default for ServerConfig {
    fn default() -> Self {
        let anvil1_url = "http://localhost:8545";
        let anvil2_url = "http://localhost:8546";
        Self {
            rpc_urls: HashMap::from([
                (MAINNET_ID, anvil1_url.to_string()),
                (SEPOLIA_ID, anvil2_url.to_string()),
            ]),
            port: 3000,
            proof_mode: ProofMode::Groth16,
        }
    }
}

impl ServerConfig {
    pub fn new(rpc_mappings: Vec<(ChainId, String)>, proof_mode: ProofMode) -> ServerConfig {
        let default = ServerConfig::default();
        ServerConfig {
            rpc_urls: if rpc_mappings.is_empty() {
                default.rpc_urls
            } else {
                rpc_mappings.into_iter().collect()
            },
            port: default.port,
            proof_mode,
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
    config.proof_mode.set_risc0_flag();
    let config = Arc::new(config);
    Router::new()
        .route("/", post(move |req| json_rpc(config, req)))
        .layer(init_trace_layer())
        // NOTE: RequestIdLayer should be added after the Trace layer
        .layer(RequestIdLayer)
}

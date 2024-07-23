use std::collections::HashMap;

use alloy_primitives::ChainId;
use vlayer_engine::evm::env::location::ExecutionLocation;

pub struct HostConfig {
    pub rpc_urls: HashMap<ChainId, String>,
    pub start_execution_location: ExecutionLocation,
}

impl HostConfig {
    pub fn new(url: &str, start_execution_location: ExecutionLocation) -> Self {
        let rpc_urls = [(start_execution_location.chain_id, url.to_string())]
            .into_iter()
            .collect();
        HostConfig {
            rpc_urls,
            start_execution_location,
        }
    }
}

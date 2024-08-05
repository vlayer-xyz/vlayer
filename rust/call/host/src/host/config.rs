use std::collections::HashMap;

use alloy_primitives::ChainId;

pub struct HostConfig {
    pub rpc_urls: HashMap<ChainId, String>,
    pub start_chain_id: ChainId,
}

impl HostConfig {
    pub fn new(rpc_urls: Vec<(ChainId, String)>, start_chain_id: ChainId) -> Self {
        let rpc_urls = rpc_urls.into_iter().collect();
        HostConfig {
            rpc_urls,
            start_chain_id,
        }
    }
}

use std::collections::HashMap;

use alloy_primitives::ChainId;

pub struct HostConfig {
    pub rpc_urls: HashMap<ChainId, String>,
    pub chain_id: ChainId,
}

impl HostConfig {
    pub fn new(url: &str, chain_id: ChainId) -> Self {
        let rpc_urls = HashMap::from([(chain_id, url.to_string())]);
        HostConfig { rpc_urls, chain_id }
    }
}

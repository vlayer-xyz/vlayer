use std::collections::HashMap;

use alloy_primitives::ChainId;

pub struct HostConfig {
    pub rpc_urls: HashMap<ChainId, String>,
    pub start_chain_id: ChainId,
}

use alloy_primitives::{BlockNumber, ChainId};
use serde::{Deserialize, Serialize};

use crate::config::{MAINNET_ID, MAINNET_MERGE_BLOCK_NUMBER};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct ExecutionLocation {
    pub block_number: BlockNumber,
    pub chain_id: ChainId,
}

impl ExecutionLocation {
    pub fn new(block_number: BlockNumber, chain_id: ChainId) -> Self {
        Self {
            block_number,
            chain_id,
        }
    }
}

impl Default for ExecutionLocation {
    fn default() -> Self {
        Self {
            chain_id: MAINNET_ID,
            block_number: MAINNET_MERGE_BLOCK_NUMBER,
        }
    }
}

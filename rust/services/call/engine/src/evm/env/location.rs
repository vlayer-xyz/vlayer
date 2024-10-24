use alloy_primitives::{BlockNumber, ChainId};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct ExecutionLocation {
    pub block_number: BlockNumber,
    pub chain_id: ChainId,
}

impl ExecutionLocation {
    pub const fn new(block_number: BlockNumber, chain_id: ChainId) -> Self {
        Self {
            block_number,
            chain_id,
        }
    }
}

impl From<(BlockNumber, ChainId)> for ExecutionLocation {
    fn from((block_number, chain_id): (BlockNumber, ChainId)) -> Self {
        Self::new(block_number, chain_id)
    }
}

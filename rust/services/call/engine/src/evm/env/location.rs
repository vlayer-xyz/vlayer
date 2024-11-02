use alloy_primitives::{BlockNumber, ChainId};
use derive_new::new;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize, new)]
pub struct ExecutionLocation {
    pub block_number: BlockNumber,
    pub chain_id: ChainId,
}

impl From<(BlockNumber, ChainId)> for ExecutionLocation {
    fn from((block_number, chain_id): (BlockNumber, ChainId)) -> Self {
        Self::new(block_number, chain_id)
    }
}

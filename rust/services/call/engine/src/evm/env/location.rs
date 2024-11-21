use alloy_primitives::{BlockNumber, ChainId};
use alloy_rlp::RlpEncodable;
use serde::{Deserialize, Serialize};

#[derive(
    Debug, Default, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize, RlpEncodable,
)]
pub struct ExecutionLocation {
    pub block_number: BlockNumber,
    pub chain_id: ChainId,
}

impl ExecutionLocation {
    #[must_use]
    pub const fn new(block_number: BlockNumber, chain_id: ChainId) -> Self {
        Self {
            block_number,
            chain_id,
        }
    }
}

impl<B, C> From<(B, C)> for ExecutionLocation
where
    B: Into<BlockNumber>,
    C: Into<ChainId>,
{
    fn from((block_number, chain_id): (B, C)) -> Self {
        ExecutionLocation {
            chain_id: chain_id.into(),
            block_number: block_number.into(),
        }
    }
}

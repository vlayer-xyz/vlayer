use alloy_primitives::{BlockNumber, ChainId};
use alloy_rlp::RlpEncodable;
use serde::{Deserialize, Serialize};

#[derive(
    Debug, Default, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize, RlpEncodable,
)]
pub struct ExecutionLocation {
    pub chain_id: ChainId,
    pub block_number: BlockNumber,
}

impl ExecutionLocation {
    #[must_use]
    pub const fn new(chain_id: ChainId, block_number: BlockNumber) -> Self {
        Self {
            chain_id,
            block_number,
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

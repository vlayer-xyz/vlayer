use alloy_primitives::{BlockNumber, ChainId};
use call_precompiles::precompile::Tag;
use serde::Serialize;

use super::ExecutionLocation;

#[derive(Debug, Copy, Clone, PartialEq, Eq, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum Metadata {
    Precompile(Precompile),
    StartChain(ChainId),
    SetChain(ExecutionLocation),
    SetBlock(ExecutionLocation),
}

impl Metadata {
    #[must_use]
    pub const fn precompile(tag: Tag, calldata_length: usize) -> Self {
        Self::Precompile(Precompile::new(tag, calldata_length))
    }

    #[must_use]
    pub const fn start_chain(chain_id: ChainId) -> Self {
        Self::StartChain(chain_id)
    }

    #[must_use]
    pub const fn set_chain(chain_id: ChainId, block_number: BlockNumber) -> Self {
        Self::SetChain(ExecutionLocation::new(chain_id, block_number))
    }

    #[must_use]
    pub const fn set_block(chain_id: ChainId, block_number: BlockNumber) -> Self {
        Self::SetBlock(ExecutionLocation::new(chain_id, block_number))
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Serialize)]
pub struct Precompile {
    pub tag: Tag,
    pub calldata_length: usize,
}

impl Precompile {
    #[must_use]
    pub const fn new(tag: Tag, calldata_length: usize) -> Self {
        Self {
            tag,
            calldata_length,
        }
    }
}

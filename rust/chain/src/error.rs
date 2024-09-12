use alloy_primitives::ChainId;
use thiserror::Error;

#[derive(Error, Debug, PartialEq)]
pub enum ChainError {
    #[error("Unsupported chain id: {0}")]
    UnsupportedChainId(ChainId),
}

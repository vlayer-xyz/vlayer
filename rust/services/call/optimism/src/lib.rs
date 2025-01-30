pub mod anchor_state_registry;
mod types;

pub mod client;

use alloy_primitives::BlockNumber;
use async_trait::async_trait;
use thiserror::Error;

use crate::types::OutputResponse;

#[derive(Debug, Error)]
pub enum ClientError {
    #[error("JsonRPSee error: {0}")]
    JsonRPSee(String),
    #[error("Requested block {requested} but client has only data for block {present}")]
    BlockNumberMismatch {
        requested: BlockNumber,
        present: BlockNumber,
    },
}

#[async_trait]
pub trait IClient: Send + Sync {
    async fn get_output_at_block(&self, block_number: u64) -> Result<OutputResponse, ClientError>;
}

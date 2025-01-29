pub mod anchor_state_registry;
mod types;

pub mod client;

use async_trait::async_trait;
use thiserror::Error;

use crate::types::OutputResponse;

#[derive(Debug, Error)]
pub enum ClientError {
    #[error("JsonRPSee error: {0}")]
    JsonRPSee(String),
}

#[async_trait]
pub trait IClient: Send + Sync {
    async fn get_output_at_block(&self, block_number: u64) -> Result<OutputResponse, ClientError>;
}

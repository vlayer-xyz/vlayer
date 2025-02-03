pub mod anchor_state_registry;
pub mod types;

pub mod client;

use async_trait::async_trait;
use thiserror::Error;

use crate::types::OutputResponse;

#[derive(Debug, Error, PartialEq, Eq)]
#[non_exhaustive]
pub enum ClientError {
    #[cfg(feature = "http")]
    #[error("Http: {0}")]
    Http(#[from] client::http::Error),
    #[error("Cached: {0}")]
    Cached(#[from] client::cached::Error),
}

#[async_trait]
pub trait IClient: Send + Sync {
    async fn get_output_at_block(&self, block_number: u64) -> Result<OutputResponse, ClientError>;
}

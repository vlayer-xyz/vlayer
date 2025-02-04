pub mod anchor_state_registry;
pub mod types;

pub mod client;

pub use alloy_eips::NumHash;
use async_trait::async_trait;
use auto_impl::auto_impl;
use thiserror::Error;

use crate::types::SequencerOutput;

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
#[auto_impl(Box)]
pub trait IClient: Send + Sync {
    async fn get_output_at_block(&self, block_number: u64) -> Result<SequencerOutput, ClientError>;
}

use alloy_primitives::ChainId;
use thiserror::Error;

use crate::IClient;

pub mod factory;
#[cfg(feature = "http")]
pub mod http;
pub mod mock;

#[derive(Debug, Error)]
pub enum FactoryError {
    #[error("HttpClientBuilder error: {0}")]
    HttpClientBuilder(String),
    #[error("No RPC URL for chain {0}")]
    NoRpcUrl(ChainId),
}

pub trait IFactory: Send + Sync {
    fn create(&self, chain_id: ChainId) -> Result<Box<dyn IClient>, FactoryError>;
}

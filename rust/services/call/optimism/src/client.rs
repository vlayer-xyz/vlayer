use alloy_primitives::ChainId;
use auto_impl::auto_impl;
use thiserror::Error;

use crate::IClient;

pub mod factory;
#[cfg(feature = "http")]
pub mod http;
pub mod mock;

#[derive(Debug, Error, PartialEq, Eq)]
pub enum FactoryError {
    #[error("HttpClientBuilder error: {0}")]
    HttpClientBuilder(String),
    #[error("No RPC URL for chain {0}")]
    NoRpcUrl(ChainId),

    #[error("No Data for chain {0}")]
    NoDataForChain(ChainId),
}

#[auto_impl(Box, &, Arc)]
pub trait IFactory: Send + Sync {
    fn create(&self, chain_id: ChainId) -> Result<Box<dyn IClient>, FactoryError>;
}

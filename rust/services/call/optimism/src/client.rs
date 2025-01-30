use alloy_primitives::ChainId;
use auto_impl::auto_impl;
use thiserror::Error;

use crate::IClient;

pub mod cached;
pub mod factory;
#[cfg(feature = "http")]
pub mod http;
pub mod recording;

#[derive(Debug, Error, PartialEq, Eq)]
pub enum FactoryError {
    #[error("Http: {0}")]
    Http(#[from] factory::http::Error),
    #[error("Mock: {0}")]
    Mock(#[from] factory::cached::Error),
}

#[auto_impl(Box, &, Arc)]
pub trait IFactory: Send + Sync {
    fn create(&self, chain_id: ChainId) -> Result<Box<dyn IClient>, FactoryError>;
}

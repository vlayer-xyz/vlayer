use alloy_primitives::ChainId;
use thiserror::Error;

use crate::rpc::{Client, DummyClient};

#[derive(Debug, Error)]
pub enum FactoryError {}

pub trait Factory: Send + Sync {
    fn create(&self, chain_id: ChainId) -> Result<Box<dyn Client>, FactoryError>;
}

pub struct DummyFactory;

impl Factory for DummyFactory {
    fn create(&self, _chain_id: ChainId) -> Result<Box<dyn Client>, FactoryError> {
        Ok(Box::new(DummyClient))
    }
}

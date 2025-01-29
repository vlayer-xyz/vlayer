use alloy_primitives::ChainId;
use thiserror::Error;

use crate::rpc::{self, IClient};

#[derive(Debug, Error)]
pub enum FactoryError {}

pub trait IFactory: Send + Sync {
    fn create(&self, chain_id: ChainId) -> Result<Box<dyn IClient>, FactoryError>;
}

pub mod mock {
    use super::*;

    pub struct Factory;

    impl IFactory for Factory {
        fn create(&self, _chain_id: ChainId) -> Result<Box<dyn IClient>, FactoryError> {
            Ok(Box::new(rpc::mock::Client))
        }
    }
}

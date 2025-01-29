use alloy_primitives::ChainId;

use crate::{
    client::{mock, FactoryError, IFactory},
    IClient,
};

pub struct Factory;

impl IFactory for Factory {
    fn create(&self, _chain_id: ChainId) -> Result<Box<dyn IClient>, FactoryError> {
        Ok(Box::new(mock::Client))
    }
}

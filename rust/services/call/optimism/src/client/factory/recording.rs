use std::{
    collections::HashMap,
    sync::{Arc, RwLock},
};

use alloy_primitives::ChainId;
use thiserror::Error;

use super::cached::OpOutputCache;
use crate::{
    IClient,
    client::{self, FactoryError, IFactory},
};

#[derive(Debug, Error, PartialEq, Eq)]
pub enum Error {
    #[error("Inner: {0}")]
    Inner(#[from] FactoryError),
}

#[derive(Clone)]
pub struct Factory {
    inner: Arc<dyn IFactory>,
    clients: Arc<RwLock<HashMap<ChainId, client::recording::Client>>>,
}

impl Factory {
    pub fn new(factory: impl IFactory + 'static) -> Self {
        Self {
            inner: Arc::new(factory),
            clients: Default::default(),
        }
    }

    #[allow(clippy::expect_used)]
    pub fn into_cache(self) -> OpOutputCache {
        let clients = Arc::try_unwrap(self.clients)
            .map_err(|_| ())
            .expect("Trying to access clients while it's still in use")
            .into_inner()
            .expect("poisoned lock");
        clients
            .into_iter()
            .map(|(k, v)| (k, v.into_cache()))
            .collect()
    }
}

impl IFactory for Factory {
    #[allow(clippy::expect_used)]
    fn create(&self, chain_id: ChainId) -> Result<Box<dyn IClient>, FactoryError> {
        let client: Box<dyn IClient> = self.inner.create(chain_id)?;
        let recording_client = client::recording::Client::new(client);
        let mut clients = self.clients.write().expect("poisoned lock");
        clients.insert(chain_id, recording_client.clone());
        Ok(Box::new(recording_client))
    }
}

use std::{collections::HashMap, sync::Arc};

use alloy_primitives::ChainId;
use derive_new::new;
use jsonrpsee::http_client::HttpClientBuilder;
use thiserror::Error;

use crate::{
    client::{http, FactoryError, IFactory},
    IClient,
};

#[derive(Debug, Error, PartialEq, Eq)]
pub enum Error {
    #[error("HttpClientBuilder error: {0}")]
    HttpClientBuilder(String),
    #[error("No RPC URL for chain {0}")]
    NoRpcUrl(ChainId),
}

#[derive(Debug, Clone, new, Default)]
pub struct Factory {
    rpc_urls: HashMap<ChainId, String>,
}

impl IFactory for Factory {
    fn create(&self, chain_id: ChainId) -> Result<Arc<dyn IClient>, FactoryError> {
        let url = self
            .rpc_urls
            .get(&chain_id)
            .ok_or(Error::NoRpcUrl(chain_id))?;
        let client = HttpClientBuilder::default()
            .build(url)
            .map_err(|err| Error::HttpClientBuilder(err.to_string()))?;
        Ok(Arc::new(http::Client::new(client)))
    }
}

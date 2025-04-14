use std::collections::HashMap;

use alloy_primitives::ChainId;
use derive_new::new;
use jsonrpsee::http_client::HttpClientBuilder;
use thiserror::Error;

use crate::{
    IClient,
    client::{FactoryError, IFactory, http},
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
    fn create(&self, chain_id: ChainId) -> Result<Box<dyn IClient>, FactoryError> {
        let url = self
            .rpc_urls
            .get(&chain_id)
            .ok_or(Error::NoRpcUrl(chain_id))?;
        let client = HttpClientBuilder::default()
            .build(url)
            .map_err(|err| Error::HttpClientBuilder(err.to_string()))?;
        Ok(Box::new(http::Client::new(client)))
    }
}

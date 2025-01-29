use std::collections::HashMap;

use alloy_primitives::ChainId;
use jsonrpsee::http_client::HttpClientBuilder;

use crate::{
    client::{http, FactoryError, IFactory},
    IClient,
};

pub struct Factory {
    rpc_urls: HashMap<ChainId, String>,
}

impl IFactory for Factory {
    fn create(&self, chain_id: ChainId) -> Result<Box<dyn IClient>, FactoryError> {
        let url = self
            .rpc_urls
            .get(&chain_id)
            .ok_or(FactoryError::NoRpcUrl(chain_id))?;
        let client = HttpClientBuilder::default()
            .build(url)
            .map_err(|err| FactoryError::HttpClientBuilder(err.to_string()))?;
        Ok(Box::new(http::Client::new(client)))
    }
}

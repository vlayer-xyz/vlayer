use std::{collections::HashMap, env};

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

impl Factory {
    fn get_rollup_endpoint_override(chain_id: ChainId) -> Option<String> {
        let env_var = match chain_id {
            10 => "OPTIMISM_ROLLUP_ENDPOINT",
            11_155_420 => "OPTIMISM_SEPOLIA_ROLLUP_ENDPOINT",
            8453 => "BASE_ROLLUP_ENDPOINT",
            84_532 => "BASE_SEPOLIA_ROLLUP_ENDPOINT",
            _ => return None,
        };

        env::var(env_var).ok()
    }
}

impl IFactory for Factory {
    fn create(&self, chain_id: ChainId) -> Result<Box<dyn IClient>, FactoryError> {
        let url = if let Some(override_url) = Self::get_rollup_endpoint_override(chain_id) {
            override_url
        } else {
            self.rpc_urls
                .get(&chain_id)
                .ok_or(Error::NoRpcUrl(chain_id))?
                .clone()
        };

        let client = HttpClientBuilder::default()
            .build(&url)
            .map_err(|err| Error::HttpClientBuilder(err.to_string()))?;
        Ok(Box::new(http::Client::new(client)))
    }
}

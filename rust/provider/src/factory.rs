use std::{collections::HashMap, path::PathBuf};

use alloy_primitives::ChainId;
use auto_impl::auto_impl;
use derive_new::new;
use thiserror::Error;
use url::ParseError;

use super::{EthersClient, cache::CachedProvider};
use crate::{BlockingProvider, EthersProvider};

#[derive(Error, Debug)]
pub enum Error {
    #[error("No rpc url for chain: {0}")]
    NoRpcUrl(ChainId),
    #[error("No rpc cache for chain: {0}")]
    NoRpcCache(ChainId),
    #[error("Failed to create cached provider: {0}")]
    CachedProvider(String),
    #[error("Failed to create rpc provider: {0}")]
    RpcProvider(#[from] ParseError),
}
pub type Result<T> = std::result::Result<T, Error>;

#[auto_impl(Box)]
pub trait ProviderFactory: Send + Sync {
    fn create(&self, chain_id: ChainId) -> Result<Box<dyn BlockingProvider>>;
}

pub struct NullProviderFactory;

impl ProviderFactory for NullProviderFactory {
    fn create(&self, _chain_id: ChainId) -> Result<Box<dyn BlockingProvider>> {
        Err(Error::CachedProvider("Forced error for testing".to_string()))
    }
}

#[derive(new)]
pub struct EthersProviderFactory {
    rpc_urls: HashMap<ChainId, String>,
}

const MAX_RETRY: u32 = 3;
const INITIAL_BACKOFF: u64 = 500;

impl ProviderFactory for EthersProviderFactory {
    fn create(&self, chain_id: ChainId) -> Result<Box<dyn BlockingProvider>> {
        let url = self
            .rpc_urls
            .get(&chain_id)
            .ok_or(Error::NoRpcUrl(chain_id))?;

        let client = EthersClient::new_client(url, MAX_RETRY, INITIAL_BACKOFF)?;

        Ok(Box::new(EthersProvider::new(client)))
    }
}

fn get_path(rpc_cache_paths: &HashMap<ChainId, String>, chain_id: ChainId) -> Result<PathBuf> {
    let path = rpc_cache_paths
        .get(&chain_id)
        .ok_or(Error::NoRpcCache(chain_id))?;

    Ok(PathBuf::from(path))
}

#[derive(new)]
pub struct CachedProviderFactory {
    rpc_cache_paths: HashMap<ChainId, String>,
    ethers_provider_factory: Option<EthersProviderFactory>,
}

impl ProviderFactory for CachedProviderFactory {
    fn create(&self, chain_id: ChainId) -> Result<Box<dyn BlockingProvider>> {
        let file_path = get_path(&self.rpc_cache_paths, chain_id)?;

        let cached_provider = match &self.ethers_provider_factory {
            Some(ethers_factory) => {
                let provider = ethers_factory.create(chain_id)?;
                CachedProvider::new(file_path, provider)
            }
            None => CachedProvider::from_file(&file_path),
        }
        .map_err(|err| Error::CachedProvider(err.to_string()))?;
        Ok(Box::new(cached_provider))
    }
}

#[cfg(test)]
mod test {
    use alloy_chains::Chain;
    use ethers_core::types::BlockNumber::Latest;

    use super::*;

    #[tokio::test(flavor = "multi_thread")]
    async fn try_new_invalid_rpc_url() -> Result<()> {
        let chain_id = Chain::mainnet().id();
        let rpc_urls = [(chain_id, "http://localhost:123".to_string())]
            .into_iter()
            .collect();
        let factory = EthersProviderFactory::new(rpc_urls);
        let provider = factory.create(chain_id)?;
        let res = provider.get_block_header(Latest);
        let error = res.unwrap_err();

        assert!(error.to_string().contains(
            "(http://localhost:123/): error trying to connect: tcp connect error: Connection refused"
        ));

        Ok(())
    }
}

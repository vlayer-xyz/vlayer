use crate::{BlockingProvider, EthersProvider};
use alloy_primitives::ChainId;
use std::collections::HashMap;
use std::path::PathBuf;
use thiserror::Error;
use url::ParseError;

use super::cache::CachedProvider;
use super::{EthersClient, FileProvider};

#[derive(Error, Debug)]
pub enum ProviderFactoryError {
    #[error("No rpc url for chain: {0}")]
    NoRpcUrl(ChainId),
    #[error("No rpc cache for chain: {0}")]
    NoRpcCache(ChainId),
    #[error("Failed to load file cache: {0}")]
    FileProvider(String),
    #[error("Failed create file cache: {0}")]
    CachedProvider(String),
    #[error("Failed to create rpc provider: {0}")]
    RpcProvider(#[from] ParseError),
}

pub trait ProviderFactory<P>
where
    P: BlockingProvider,
{
    fn create(&self, chain_id: ChainId) -> Result<P, ProviderFactoryError>;
}

pub struct EthersProviderFactory {
    rpc_urls: HashMap<ChainId, String>,
}

const MAX_RETRY: u32 = 3;
const INITIAL_BACKOFF: u64 = 500;

impl EthersProviderFactory {
    pub fn new(rpc_urls: HashMap<ChainId, String>) -> Self {
        EthersProviderFactory { rpc_urls }
    }
}

impl ProviderFactory<EthersProvider<EthersClient>> for EthersProviderFactory {
    fn create(
        &self,
        chain_id: ChainId,
    ) -> Result<EthersProvider<EthersClient>, ProviderFactoryError> {
        let url = self
            .rpc_urls
            .get(&chain_id)
            .ok_or(ProviderFactoryError::NoRpcUrl(chain_id))?;

        let client = EthersClient::new_client(url, MAX_RETRY, INITIAL_BACKOFF)?;

        Ok(EthersProvider::new(client))
    }
}

fn get_path(
    rpc_file_cache: &HashMap<ChainId, String>,
    chain_id: ChainId,
) -> Result<PathBuf, ProviderFactoryError> {
    let file_path_str = rpc_file_cache
        .get(&chain_id)
        .ok_or(ProviderFactoryError::NoRpcCache(chain_id))?;

    Ok(PathBuf::from(file_path_str))
}

pub struct FileProviderFactory {
    rpc_file_cache: HashMap<ChainId, String>,
}

impl FileProviderFactory {
    pub fn new(rpc_file_cache: HashMap<ChainId, String>) -> Self {
        FileProviderFactory { rpc_file_cache }
    }
}

impl ProviderFactory<FileProvider> for FileProviderFactory {
    fn create(&self, chain_id: ChainId) -> Result<FileProvider, ProviderFactoryError> {
        let file_path = get_path(&self.rpc_file_cache, chain_id)?;

        FileProvider::from_file(&file_path)
            .map_err(|err| ProviderFactoryError::FileProvider(err.to_string()))
    }
}

pub struct CachedProviderFactory {
    ethers_provider_factory: EthersProviderFactory,
    rpc_file_cache: HashMap<ChainId, String>,
}

impl CachedProviderFactory {
    pub fn new(
        rpc_urls: HashMap<ChainId, String>,
        rpc_file_cache: HashMap<ChainId, String>,
    ) -> Self {
        CachedProviderFactory {
            ethers_provider_factory: EthersProviderFactory::new(rpc_urls),
            rpc_file_cache,
        }
    }
}

impl ProviderFactory<CachedProvider<EthersProvider<EthersClient>>> for CachedProviderFactory {
    fn create(
        &self,
        chain_id: ChainId,
    ) -> Result<CachedProvider<EthersProvider<EthersClient>>, ProviderFactoryError> {
        let file_path = get_path(&self.rpc_file_cache, chain_id)?;

        let provider = self.ethers_provider_factory.create(chain_id)?;
        CachedProvider::new(file_path, provider)
            .map_err(|err| ProviderFactoryError::CachedProvider(err.to_string()))
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use alloy_chains::Chain;
    use ethers_core::types::BlockNumber::Latest;

    #[test]
    fn try_new_invalid_rpc_url() -> anyhow::Result<()> {
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

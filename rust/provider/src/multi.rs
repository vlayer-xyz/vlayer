use std::{
    collections::HashMap,
    sync::{Arc, RwLock},
};

use alloy_primitives::{Address, BlockNumber, Bytes, ChainId};
use block_header::EvmBlockHeader;
use common::InteriorMutabilityCache;
use ethers_core::types::BlockNumber as BlockTag;
use thiserror::Error;

use crate::{BlockingProvider, NullProviderFactory, ProviderFactory, factory};

type MultiProvider = HashMap<ChainId, Arc<dyn BlockingProvider>>;

#[derive(Clone)]
pub struct CachedMultiProvider {
    cache: Arc<RwLock<MultiProvider>>,
    factory: Arc<dyn ProviderFactory>,
}

#[derive(Debug, Error)]
pub enum Error {
    #[error("Provider: {0}")]
    Provider(#[from] crate::Error),
    #[error("Provider factory: {0}")]
    ProviderFactory(#[from] factory::Error),
    #[error("Block not found: {0}")]
    BlockNotFound(BlockTag),
}
pub type Result<T> = std::result::Result<T, Error>;

impl CachedMultiProvider {
    pub fn new(
        factory: impl ProviderFactory + 'static,
        providers: impl IntoIterator<Item = (ChainId, Arc<dyn BlockingProvider>)>,
    ) -> Self {
        CachedMultiProvider {
            cache: Arc::new(RwLock::new(HashMap::from_iter(providers))),
            factory: Arc::new(factory),
        }
    }

    pub fn from_factory(factory: impl ProviderFactory + 'static) -> Self {
        CachedMultiProvider::new(factory, [])
    }

    pub fn from_providers(
        providers: impl IntoIterator<Item = (ChainId, Arc<dyn BlockingProvider>)>,
    ) -> Self {
        CachedMultiProvider::new(NullProviderFactory, providers)
    }

    pub fn from_provider(chain_id: ChainId, provider: Arc<dyn BlockingProvider>) -> Self {
        CachedMultiProvider::from_providers([(chain_id, provider)])
    }

    pub fn get(&self, chain_id: ChainId) -> Result<Arc<dyn BlockingProvider>> {
        Ok(self
            .cache
            .try_get_or_insert(chain_id, || self.factory.create(chain_id))?)
    }

    pub fn get_latest_block_number(&self, chain_id: ChainId) -> Result<BlockNumber> {
        let provider = self.get(chain_id)?;
        Ok(provider.get_latest_block_number()?)
    }

    pub fn get_block_header(
        &self,
        chain_id: ChainId,
        block_num: BlockTag,
    ) -> Result<Box<dyn EvmBlockHeader>> {
        let provider = self.get(chain_id)?;

        let block_header = provider
            .get_block_header(block_num)?
            .ok_or(Error::BlockNotFound(block_num))?;

        Ok(block_header)
    }

    pub fn get_code(
        &self,
        chain_id: ChainId,
        address: Address,
        block_num: BlockNumber,
    ) -> Result<Bytes> {
        let provider = self.get(chain_id)?;
        Ok(provider.get_code(address, block_num)?)
    }
}

#[cfg(test)]
mod get {
    use std::path::PathBuf;

    use alloy_chains::Chain;

    use super::*;
    use crate::{CachedProviderFactory, cache::CachedProvider};

    #[test]
    fn gets_cached_provider() -> anyhow::Result<()> {
        let chain_id = Chain::mainnet().id();
        let file_path = PathBuf::from("testdata/cache.json");
        let provider =
            Arc::new(CachedProvider::from_file(&file_path)?) as Arc<dyn BlockingProvider>;

        // NullProviderFactory returns an error when it tries to create a provider.
        // If no error was returned, it means the factory did not try to create a provider and used cached provider.
        let cached_multi_provider = CachedMultiProvider::from_provider(chain_id, provider.clone());

        let returned_provider = cached_multi_provider.get(chain_id)?;

        assert!(Arc::ptr_eq(&provider, &returned_provider));

        Ok(())
    }

    #[test]
    fn gets_created_provider() -> anyhow::Result<()> {
        let rpc_cache_path =
            HashMap::from([(Chain::mainnet().id(), "testdata/cache.json".to_string())]);

        let provider_factory = CachedProviderFactory::new(rpc_cache_path, None);
        let cached_multi_provider = CachedMultiProvider::from_factory(provider_factory);
        cached_multi_provider.get(Chain::mainnet().id())?;

        Ok(())
    }
}

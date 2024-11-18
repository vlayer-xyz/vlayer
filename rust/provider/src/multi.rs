use std::{
    collections::HashMap,
    sync::{Arc, RwLock},
};

use alloy_primitives::ChainId;
use common::InteriorMutabilityCache;

use crate::{
    factory::ProviderFactoryError, BlockingProvider, NullProviderFactory, ProviderFactory,
};

type MultiProvider = HashMap<ChainId, Arc<dyn BlockingProvider>>;

pub struct CachedMultiProvider {
    cache: RwLock<MultiProvider>,
    factory: Box<dyn ProviderFactory>,
}

impl CachedMultiProvider {
    pub fn new(
        factory: impl ProviderFactory + 'static,
        providers: impl IntoIterator<Item = (ChainId, Arc<dyn BlockingProvider>)>,
    ) -> Self {
        CachedMultiProvider {
            cache: RwLock::new(HashMap::from_iter(providers)),
            factory: Box::new(factory),
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

    pub fn get(
        &self,
        chain_id: ChainId,
    ) -> Result<Arc<dyn BlockingProvider>, ProviderFactoryError> {
        self.cache
            .try_get_or_insert(chain_id, || self.factory.create(chain_id))
    }
}

#[cfg(test)]
mod get {
    use std::path::PathBuf;

    use alloy_chains::Chain;
    use null_provider_factory::NullProviderFactory;

    use super::*;
    use crate::{cache::CachedProvider, CachedProviderFactory};

    #[test]
    fn gets_cached_provider() -> anyhow::Result<()> {
        let chain_id = Chain::mainnet().id();
        let path_buf = PathBuf::from("testdata/cache.json");
        let provider = Arc::new(CachedProvider::from_file(&path_buf)?) as Arc<dyn BlockingProvider>;

        // NullProviderFactory returns an error when it tries to create a provider.
        // If no error was returned, it means the factory did not try to create a provider and used cached provider.
        let cached_multi_provider = CachedMultiProvider::from_provider(chain_id, provider.clone());

        let returned_provider = cached_multi_provider.get(chain_id)?;

        assert!(Arc::ptr_eq(&provider, &returned_provider));

        Ok(())
    }

    #[test]
    fn gets_created_provider() -> anyhow::Result<()> {
        let rpc_file_cache =
            HashMap::from([(Chain::mainnet().id(), "testdata/cache.json".to_string())]);

        let provider_factory = CachedProviderFactory::new(rpc_file_cache, None);
        let cached_multi_provider = CachedMultiProvider::from_factory(provider_factory);
        cached_multi_provider.get(Chain::mainnet().id())?;

        Ok(())
    }
}

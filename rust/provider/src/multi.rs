use std::{
    collections::HashMap,
    sync::{Arc, RwLock},
};

use alloy_primitives::ChainId;
use call_engine::utils::InteriorMutabilityCache;

use super::{factory::ProviderFactory, BlockingProvider};
use crate::factory::ProviderFactoryError;

type MultiProvider = HashMap<ChainId, Arc<Box<dyn BlockingProvider>>>;

pub struct CachedMultiProvider {
    cache: RwLock<MultiProvider>,
    factory: Box<dyn ProviderFactory>,
}

impl CachedMultiProvider {
    pub fn new(factory: Box<dyn ProviderFactory>) -> Self {
        CachedMultiProvider {
            cache: RwLock::new(HashMap::new()),
            factory,
        }
    }

    pub fn get(
        &self,
        chain_id: ChainId,
    ) -> Result<Arc<Box<dyn BlockingProvider>>, ProviderFactoryError> {
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
    use crate::{cache::CachedProvider, factory::FileProviderFactory};

    mod null_provider_factory {
        use super::*;

        pub(crate) struct NullProviderFactory;

        impl ProviderFactory for NullProviderFactory {
            fn create(
                &self,
                _chain_id: ChainId,
            ) -> Result<Box<dyn BlockingProvider>, ProviderFactoryError> {
                Err(ProviderFactoryError::CachedProvider("Forced error for testing".to_string()))
            }
        }
    }

    #[test]
    fn gets_cached_provider() -> anyhow::Result<()> {
        let path_buf = PathBuf::from("testdata/cache.json");
        let provider =
            Arc::new(Box::new(CachedProvider::from_file(&path_buf)?) as Box<dyn BlockingProvider>);

        let cache = RwLock::new(HashMap::from([(Chain::mainnet().id(), Arc::clone(&provider))]));

        // NullProviderFactory returns an error when it tries to create a provider.
        // If no error was returned, it means the factory did not try to create a provider and used cached provider.
        let cached_multi_provider = CachedMultiProvider {
            cache,
            factory: Box::new(NullProviderFactory {}),
        };

        let returned_provider = cached_multi_provider.get(Chain::mainnet().id())?;

        assert!(Arc::ptr_eq(&provider, &returned_provider));

        Ok(())
    }

    #[test]
    fn gets_created_provider() -> anyhow::Result<()> {
        let rpc_file_cache =
            HashMap::from([(Chain::mainnet().id(), "testdata/cache.json".to_string())]);

        let provider_factory = FileProviderFactory::new(rpc_file_cache);
        let cached_multi_provider = CachedMultiProvider::new(Box::new(provider_factory));
        cached_multi_provider.get(Chain::mainnet().id())?;

        Ok(())
    }
}

use super::factory::ProviderFactory;
use super::BlockingProvider;
use crate::host::error::HostError;
use alloy_primitives::ChainId;
use std::cell::RefCell;
use std::{collections::HashMap, rc::Rc};
use vlayer_engine::utils::InteriorMutabilityCache;

type MultiProvider<P> = HashMap<ChainId, Rc<P>>;

pub struct CachedMultiProvider<P> {
    cache: RefCell<MultiProvider<P>>,
    factory: Box<dyn ProviderFactory<P>>,
}

impl<P> CachedMultiProvider<P>
where
    P: BlockingProvider,
{
    pub fn new(factory: impl ProviderFactory<P> + 'static) -> Self {
        CachedMultiProvider {
            cache: RefCell::new(HashMap::new()),
            factory: Box::new(factory),
        }
    }

    pub fn get(&self, chain_id: ChainId) -> Result<Rc<P>, HostError> {
        self.cache
            .try_get_or_insert(chain_id, || self.factory.create(chain_id))
    }
}

#[cfg(test)]
mod get {
    use crate::provider::{factory::FileProviderFactory, FileProvider};

    use super::*;
    use null_provider_factory::NullProviderFactory;
    use std::path::PathBuf;
    use vlayer_engine::config::MAINNET_ID;

    mod null_provider_factory {
        use super::{HostError, ProviderFactory};
        use crate::provider::FileProvider;
        use alloy_primitives::ChainId;

        pub struct NullProviderFactory;

        impl ProviderFactory<FileProvider> for NullProviderFactory {
            fn create(&self, _chain_id: ChainId) -> Result<FileProvider, HostError> {
                Err(HostError::Provider("Forced error for testing".to_string()))
            }
        }
    }

    #[test]
    fn gets_cached_provider() -> anyhow::Result<()> {
        let path_buf = PathBuf::from("testdata/mainnet_uniswap_factory_owner_rpc_cache.json");
        let provider = Rc::new(FileProvider::from_file(&path_buf)?);

        let cache = RefCell::new(HashMap::from([(MAINNET_ID, Rc::clone(&provider))]));

        // NullProviderFactory returns an error when it tries to create a provider.
        // If no error was returned, it means the factory did not try to create a provider and used cached provider.
        let cached_multi_provider = CachedMultiProvider {
            cache,
            factory: Box::new(NullProviderFactory {}),
        };

        let returned_provider = cached_multi_provider.get(MAINNET_ID)?;

        assert_eq!(*provider, *returned_provider);

        Ok(())
    }

    #[test]
    fn gets_created_provider() -> anyhow::Result<()> {
        let rpc_file_cache = HashMap::from([(
            MAINNET_ID,
            "testdata/mainnet_uniswap_factory_owner_rpc_cache.json".to_string(),
        )]);

        let provider_factory = FileProviderFactory::new(rpc_file_cache);
        let cached_multi_provider = CachedMultiProvider::new(provider_factory);
        cached_multi_provider.get(MAINNET_ID)?;

        Ok(())
    }
}

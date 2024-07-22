use super::{factory::ProviderFactory, Provider};
use crate::host::error::HostError;
use alloy_primitives::ChainId;
use std::cell::RefCell;
use std::{collections::HashMap, rc::Rc};
use vlayer_engine::utils::TryGetOrInsert;

type MultiProvider<P> = HashMap<ChainId, Rc<P>>;

pub struct CachedMultiProvider<P> {
    cache: RefCell<MultiProvider<P>>,
    factory: Box<dyn ProviderFactory<P>>,
}

impl<P> CachedMultiProvider<P>
where
    P: Provider,
{
    pub fn new(factory: impl ProviderFactory<P> + 'static) -> Self {
        CachedMultiProvider {
            cache: RefCell::new(HashMap::new()),
            factory: Box::new(factory),
        }
    }

    pub fn get(&self, chain_id: ChainId) -> Result<Rc<P>, HostError> {
        let create_provider = || Ok::<_, HostError>(Rc::new(self.factory.create(chain_id)?));
        let mut mut_cache = self.cache.borrow_mut();
        let provider = mut_cache.try_get_or_insert(chain_id, create_provider);
        let cloned_provider = Rc::clone(provider?);
        Ok(cloned_provider)
    }
}

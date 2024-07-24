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

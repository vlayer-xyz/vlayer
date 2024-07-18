use super::{factory::ProviderFactory, Provider};
use crate::{host::error::HostError, utils::TryGetOrInsert};
use alloy_primitives::ChainId;
use std::{collections::HashMap, rc::Rc};

type MultiProvider<P> = HashMap<ChainId, Rc<P>>;

pub struct CachedMultiProvider<P> {
    cache: MultiProvider<P>,
    factory: Box<dyn ProviderFactory<P>>,
}

impl<P> CachedMultiProvider<P>
where
    P: Provider,
{
    pub fn new(factory: impl ProviderFactory<P> + 'static) -> Self {
        CachedMultiProvider {
            cache: HashMap::new(),
            factory: Box::new(factory),
        }
    }

    pub fn get(&mut self, chain_id: ChainId) -> Result<Rc<P>, HostError> {
        let create_provider = || Ok::<_, HostError>(Rc::new(self.factory.create(chain_id)?));
        Ok(Rc::clone(
            self.cache.try_get_or_insert(chain_id, create_provider)?,
        ))
    }
}

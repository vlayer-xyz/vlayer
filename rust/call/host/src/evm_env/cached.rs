use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;

use call_engine::evm::env::{
    cached::MultiEvmEnv, location::ExecutionLocation, EvmEnv, EvmEnvFactory,
};
use call_engine::utils::InteriorMutabilityCache;

use crate::db::proof::ProofDb;
use crate::provider::BlockingProvider;

pub(crate) struct CachedEvmEnv<P>
where
    P: BlockingProvider,
{
    cache: MultiEvmEnv<ProofDb<P>>,
    factory: Box<dyn EvmEnvFactory<ProofDb<P>>>,
}

impl<P> CachedEvmEnv<P>
where
    P: BlockingProvider,
{
    pub(crate) fn new(factory: impl EvmEnvFactory<ProofDb<P>> + 'static) -> Self {
        CachedEvmEnv {
            cache: RefCell::new(HashMap::new()),
            factory: Box::new(factory),
        }
    }

    pub(crate) fn get(
        &mut self,
        location: ExecutionLocation,
    ) -> Result<Rc<EvmEnv<ProofDb<P>>>, anyhow::Error> {
        self.cache
            .try_get_or_insert(location, || self.factory.create(location))
    }

    pub(crate) fn into_inner(self) -> MultiEvmEnv<ProofDb<P>> {
        self.cache
    }
}

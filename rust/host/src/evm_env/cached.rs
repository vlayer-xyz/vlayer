use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;

use vlayer_engine::evm::env::{location::ExecutionLocation, EvmEnv};

use crate::{db::proof::ProofDb, provider::BlockingProvider};

pub struct CachedEvmEnv<P>
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
    pub fn new(factory: impl EvmEnvFactory<ProofDb<P>> + 'static) -> Self {
        CachedEvmEnv {
            cache: RefCell::new(HashMap::new()),
            factory: Box::new(factory),
        }
    }

    pub fn get(
        &mut self,
        location: ExecutionLocation,
    ) -> Result<Rc<EvmEnv<ProofDb<P>>>, anyhow::Error> {
        self.cache
            .try_get_or_insert(location, || self.factory.create(location))
    }

    pub fn into_inner(self) -> MultiEvmEnv<ProofDb<P>> {
        self.cache
    }
}

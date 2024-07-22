use vlayer_engine::evm::env::{EvmEnv, ExecutionLocation, MultiEvmEnv};

use crate::{
    db::proof::ProofDb, host::error::HostError, provider::Provider, utils::TryGetOrInsert,
};

use super::factory::EvmEnvFactory;

pub struct CachedEvmEnv<P>
where
    P: Provider,
{
    cache: MultiEvmEnv<ProofDb<P>, P::Header>,
    factory: Box<dyn EvmEnvFactory<P>>,
}

impl<P> CachedEvmEnv<P>
where
    P: Provider,
{
    pub fn new(factory: Box<dyn EvmEnvFactory<P>>) -> Self {
        CachedEvmEnv {
            cache: MultiEvmEnv::new(),
            factory,
        }
    }

    pub fn get(
        &mut self,
        location: ExecutionLocation,
    ) -> Result<&EvmEnv<ProofDb<P>, P::Header>, HostError> {
        self.cache
            .try_get_or_insert(location, || self.factory.create(location))
    }

    pub fn into_inner(self) -> MultiEvmEnv<ProofDb<P>, P::Header> {
        self.cache
    }
}

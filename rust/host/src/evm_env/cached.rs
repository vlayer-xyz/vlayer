use vlayer_engine::evm::env::{EvmEnv, EvmEnvFactory, ExecutionLocation, MultiEvmEnv};

use crate::{
    db::proof::ProofDb, host::error::HostError, provider::Provider, utils::TryGetOrInsert,
};

pub struct CachedEvmEnv<P>
where
    P: Provider,
{
    cache: MultiEvmEnv<ProofDb<P>, P::Header>,
    factory: Box<dyn EvmEnvFactory<ProofDb<P>, P::Header>>,
}

impl<P> CachedEvmEnv<P>
where
    P: Provider,
{
    pub fn new(factory: Box<dyn EvmEnvFactory<ProofDb<P>, P::Header>>) -> Self {
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
            .map_err(HostError::EvmEnvFactory)
    }

    pub fn into_inner(self) -> MultiEvmEnv<ProofDb<P>, P::Header> {
        self.cache
    }
}

use vlayer_engine::evm::env::{EvmEnv, ExecutionLocation, MultiEvmEnv};

use crate::{
    db::proof::ProofDb, host::error::HostError, provider::Provider,
    utils::get_mut_or_insert_with_result,
};

use super::factory::EvmEnvFactory;

pub struct CachedEvmEnv<P>
where
    P: Provider,
{
    cache: MultiEvmEnv<ProofDb<P>, P::Header>,
    factory: EvmEnvFactory<P>,
}

impl<P> CachedEvmEnv<P>
where
    P: Provider,
{
    pub fn new(factory: EvmEnvFactory<P>) -> Self {
        CachedEvmEnv {
            cache: MultiEvmEnv::new(),
            factory,
        }
    }

    pub fn get(
        &mut self,
        location: ExecutionLocation,
    ) -> Result<&mut EvmEnv<ProofDb<P>, P::Header>, HostError> {
        get_mut_or_insert_with_result(&mut self.cache, location, || self.factory.create(location))
    }

    pub fn into_inner(self) -> MultiEvmEnv<ProofDb<P>, P::Header> {
        self.cache
    }
}

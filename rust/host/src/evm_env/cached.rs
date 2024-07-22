use revm::DatabaseRef;
use vlayer_engine::{
    block_header::EvmBlockHeader,
    evm::env::{EvmEnv, EvmEnvFactory, ExecutionLocation, MultiEvmEnv},
};

use crate::utils::TryGetOrInsert;

pub struct CachedEvmEnv<D, H>
where
    D: DatabaseRef,
    H: EvmBlockHeader,
{
    cache: MultiEvmEnv<D, H>,
    factory: Box<dyn EvmEnvFactory<D, H>>,
}

impl<D, H> CachedEvmEnv<D, H>
where
    D: DatabaseRef,
    H: EvmBlockHeader,
{
    pub fn new(factory: Box<dyn EvmEnvFactory<D, H>>) -> Self {
        CachedEvmEnv {
            cache: MultiEvmEnv::new(),
            factory,
        }
    }

    pub fn get(&mut self, location: ExecutionLocation) -> anyhow::Result<&EvmEnv<D, H>> {
        self.cache
            .try_get_or_insert(location, || self.factory.create(location))
    }

    pub fn into_inner(self) -> MultiEvmEnv<D, H> {
        self.cache
    }
}

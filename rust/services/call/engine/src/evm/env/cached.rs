use std::{
    collections::HashMap,
    fmt::{self, Debug, Formatter},
    sync::{Arc, Mutex, RwLock},
};

use alloy_primitives::ChainId;
use call_common::{ExecutionLocation, RevmDB};
use common::InteriorMutabilityCache;
use itertools::Itertools;

use super::{
    BlocksByChain, EvmEnv,
    factory::{Error, EvmEnvFactory, Result},
};

pub struct NullEvmEnvFactory;

impl<D: RevmDB> EvmEnvFactory<D> for NullEvmEnvFactory {
    fn create(&self, _location: ExecutionLocation) -> Result<EvmEnv<D>> {
        Err(Error::NullEvmEnvFactory)
    }
}

pub type MultiEvmEnv<D> = RwLock<HashMap<ExecutionLocation, Arc<EvmEnv<D>>>>;

pub struct CachedEvmEnv<D: RevmDB> {
    cache: MultiEvmEnv<D>,
    // Mutex makes it UnwindSafe
    factory: Mutex<Box<dyn EvmEnvFactory<D>>>,
}

impl<D: RevmDB> Debug for CachedEvmEnv<D> {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        f.debug_struct("CachedEvmEnv")
            .field("cache", &self.cache)
            .finish()
    }
}

impl<D: RevmDB> CachedEvmEnv<D> {
    pub fn from_factory(factory: impl EvmEnvFactory<D> + 'static) -> Self {
        CachedEvmEnv {
            cache: RwLock::new(HashMap::new()),
            factory: Mutex::new(Box::new(factory)),
        }
    }

    pub fn from_envs(envs: MultiEvmEnv<D>) -> Self {
        CachedEvmEnv {
            cache: envs,
            factory: Mutex::new(Box::new(NullEvmEnvFactory)),
        }
    }

    #[allow(clippy::len_without_is_empty)]
    pub fn len(&self) -> usize {
        self.cache.read().expect("poisoned lock").len()
    }

    pub fn get(&self, location: ExecutionLocation) -> Result<Arc<EvmEnv<D>>> {
        self.cache.try_get_or_insert(location, || {
            self.factory.lock().expect("poisoned lock").create(location)
        })
    }

    pub fn into_inner(self) -> HashMap<ExecutionLocation, Arc<EvmEnv<D>>> {
        self.cache.into_inner().expect("poisoned lock")
    }

    fn group_blocks<F, T>(&self, f: F) -> HashMap<ChainId, Vec<T>>
    where
        F: Fn(&ExecutionLocation, &EvmEnv<D>) -> T,
    {
        let cache = self.cache.read().expect("poisoned lock");
        cache
            .iter()
            .map(|(loc, evm_env)| (loc.chain_id, f(loc, evm_env)))
            .into_group_map()
    }

    pub fn blocks_by_chain(&self) -> BlocksByChain {
        self.group_blocks(|loc, evm_env| (loc.block_number, evm_env.header.hash_slow()))
            .into()
    }
}

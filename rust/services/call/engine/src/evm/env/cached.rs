use std::{
    collections::HashMap,
    sync::{Arc, Mutex, RwLock},
};

use common::InteriorMutabilityCache;
use revm::DatabaseRef;

use super::{
    factory::{Error, EvmEnvFactory, Result},
    location::ExecutionLocation,
    EvmEnv,
};

pub struct NullEvmEnvFactory;

impl<D> EvmEnvFactory<D> for NullEvmEnvFactory
where
    D: DatabaseRef,
{
    fn create(&self, _location: ExecutionLocation) -> Result<EvmEnv<D>> {
        Err(Error::NullEvmEnvFactory)
    }
}

pub type MultiEvmEnv<D> = RwLock<HashMap<ExecutionLocation, Arc<EvmEnv<D>>>>;

pub struct CachedEvmEnv<D>
where
    D: DatabaseRef,
{
    cache: MultiEvmEnv<D>,
    // Mutex makes it UnwindSafe
    factory: Mutex<Box<dyn EvmEnvFactory<D>>>,
}

impl<D> CachedEvmEnv<D>
where
    D: DatabaseRef,
{
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

    pub fn get(&self, location: ExecutionLocation) -> Result<Arc<EvmEnv<D>>> {
        self.cache.try_get_or_insert(location, || {
            self.factory.lock().expect("poisoned lock").create(location)
        })
    }

    pub fn into_inner(self) -> HashMap<ExecutionLocation, Arc<EvmEnv<D>>> {
        self.cache.into_inner().expect("poisoned lock")
    }
}

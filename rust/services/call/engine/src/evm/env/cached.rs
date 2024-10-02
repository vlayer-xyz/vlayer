use std::{cell::RefCell, collections::HashMap, rc::Rc};

use anyhow::bail;
use revm::DatabaseRef;

use super::{location::ExecutionLocation, EvmEnv, EvmEnvFactory};
use crate::utils::InteriorMutabilityCache;

pub struct NullEvmEnvFactory;

impl<D> EvmEnvFactory<D> for NullEvmEnvFactory
where
    D: DatabaseRef,
{
    fn create(&self, _location: ExecutionLocation) -> anyhow::Result<EvmEnv<D>> {
        bail!("NullEvmEnvFactory cannot create EvmEnv")
    }
}

pub type MultiEvmEnv<D> = RefCell<HashMap<ExecutionLocation, Rc<EvmEnv<D>>>>;

pub struct CachedEvmEnv<D>
where
    D: DatabaseRef,
{
    cache: MultiEvmEnv<D>,
    factory: Box<dyn EvmEnvFactory<D>>,
}

impl<D> CachedEvmEnv<D>
where
    D: DatabaseRef,
{
    pub fn from_factory(factory: impl EvmEnvFactory<D> + 'static) -> Self {
        CachedEvmEnv {
            cache: RefCell::new(HashMap::new()),
            factory: Box::new(factory),
        }
    }

    pub fn from_envs(envs: MultiEvmEnv<D>) -> Self {
        CachedEvmEnv {
            cache: envs,
            factory: Box::new(NullEvmEnvFactory),
        }
    }

    pub fn get(&self, location: ExecutionLocation) -> anyhow::Result<Rc<EvmEnv<D>>> {
        self.cache
            .try_get_or_insert(location, || self.factory.create(location))
    }

    pub fn into_inner(self) -> HashMap<ExecutionLocation, Rc<EvmEnv<D>>> {
        self.cache.into_inner()
    }
}

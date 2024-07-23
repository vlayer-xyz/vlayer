use std::{cell::RefCell, collections::HashMap, rc::Rc};

use anyhow::bail;
use revm::DatabaseRef;

use crate::{block_header::EvmBlockHeader, utils::InteriorMutabilityCache};

use super::{location::ExecutionLocation, EvmEnv, EvmEnvFactory};

pub struct NullEvmEnvFactory;

impl<D, H> EvmEnvFactory<D, H> for NullEvmEnvFactory
where
    D: DatabaseRef,
    H: EvmBlockHeader,
{
    fn create(&self, _location: ExecutionLocation) -> anyhow::Result<EvmEnv<D, H>> {
        bail!("NullEvmEnvFactory cannot create EvmEnv")
    }
}

pub type MultiEvmEnv<D, H> = RefCell<HashMap<ExecutionLocation, Rc<EvmEnv<D, H>>>>;

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
    pub fn from_factory(factory: Box<dyn EvmEnvFactory<D, H>>) -> Self {
        CachedEvmEnv {
            cache: RefCell::new(HashMap::new()),
            factory,
        }
    }

    pub fn from_envs(envs: MultiEvmEnv<D, H>) -> Self {
        CachedEvmEnv {
            cache: envs,
            factory: Box::new(NullEvmEnvFactory),
        }
    }

    pub fn get(&self, location: ExecutionLocation) -> anyhow::Result<Rc<EvmEnv<D, H>>> {
        self.cache
            .try_get_or_insert(location, || self.factory.create(location))
    }

    pub fn into_inner(self) -> HashMap<ExecutionLocation, Rc<EvmEnv<D, H>>> {
        self.cache.into_inner()
    }
}

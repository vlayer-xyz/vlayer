use alloy_primitives::{BlockNumber, ChainId};
use anyhow::bail;
use revm::{
    primitives::{CfgEnvWithHandlerCfg, SpecId},
    DatabaseRef,
};
use serde::{Deserialize, Serialize};
use std::{cell::RefCell, collections::HashMap, rc::Rc};

use crate::{
    block_header::EvmBlockHeader, chain::spec::ChainSpec, engine::EngineError,
    utils::InteriorMutabilityCache,
};

/// The environment to execute the contract calls in.
pub struct EvmEnv<D, H> {
    pub db: D,
    pub cfg_env: CfgEnvWithHandlerCfg,
    pub header: H,
}

impl<D, H: EvmBlockHeader> EvmEnv<D, H> {
    /// Creates a new environment.
    /// It uses the default configuration for the latest specification.
    pub fn new(db: D, header: H) -> Self {
        let cfg_env = CfgEnvWithHandlerCfg::new_with_spec_id(Default::default(), SpecId::LATEST);

        Self {
            db,
            cfg_env,
            header,
        }
    }

    /// Sets the chain ID and specification ID from the given chain spec.
    pub fn with_chain_spec(mut self, chain_spec: &ChainSpec) -> Result<Self, EngineError> {
        self.cfg_env.chain_id = chain_spec.chain_id();
        self.cfg_env.handler_cfg.spec_id = chain_spec
            .active_fork(self.header.number(), self.header.timestamp())
            .map_err(|err| EngineError::ChainSpecError(err.to_string()))?;
        Ok(self)
    }

    /// Returns the header of the environment.
    pub fn header(&self) -> &H {
        &self.header
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct ExecutionLocation {
    pub block_number: BlockNumber,
    pub chain_id: ChainId,
}

impl ExecutionLocation {
    pub fn new(block_number: BlockNumber, chain_id: ChainId) -> Self {
        Self {
            block_number,
            chain_id,
        }
    }
}

pub trait EvmEnvFactory<D, H>
where
    D: DatabaseRef,
    H: EvmBlockHeader,
{
    fn create(&self, location: ExecutionLocation) -> anyhow::Result<EvmEnv<D, H>>;
}

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
    pub fn new(factory: Box<dyn EvmEnvFactory<D, H>>) -> Self {
        CachedEvmEnv {
            cache: RefCell::new(HashMap::new()),
            factory,
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

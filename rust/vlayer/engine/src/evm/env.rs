use alloy_primitives::{BlockNumber, ChainId, Sealed};
use revm::primitives::{CfgEnvWithHandlerCfg, SpecId};
use std::collections::HashMap;

use crate::{chain::spec::ChainSpec, engine::EngineError};

use super::block_header::EvmBlockHeader;

/// The environment to execute the contract calls in.
pub struct EvmEnv<D, H> {
    pub db: D,
    pub cfg_env: CfgEnvWithHandlerCfg,
    pub header: Sealed<H>,
}

impl<D, H: EvmBlockHeader> EvmEnv<D, H> {
    /// Creates a new environment.
    /// It uses the default configuration for the latest specification.
    pub fn new(db: D, header: Sealed<H>) -> Self {
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
        self.header.inner()
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct ExecutionLocation {
    block_number: BlockNumber,
    chain_id: ChainId,
}

impl ExecutionLocation {
    pub fn new(block_number: BlockNumber, chain_id: ChainId) -> Self {
        Self {
            block_number,
            chain_id,
        }
    }

    pub fn block_number(&self) -> BlockNumber {
        self.block_number
    }

    pub fn chain_id(&self) -> ChainId {
        self.chain_id
    }
}

pub struct MultiEnv<D, H> {
    pub envs: HashMap<ExecutionLocation, EvmEnv<D, H>>,
}

impl<D, H> Default for MultiEnv<D, H> {
    fn default() -> Self {
        Self {
            envs: HashMap::new(),
        }
    }
}

impl<D, H: EvmBlockHeader> MultiEnv<D, H> {
    pub fn insert(&mut self, location: ExecutionLocation, env: EvmEnv<D, H>) {
        self.envs.insert(location, env);
    }

    pub fn get_mut(
        &mut self,
        location: &ExecutionLocation,
    ) -> Result<&mut EvmEnv<D, H>, EngineError> {
        self.envs
            .get_mut(location)
            .ok_or(EngineError::EvmNotFound(*location))
    }
}

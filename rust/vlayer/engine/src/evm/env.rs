use alloy_primitives::{BlockNumber, ChainId, Sealed};
use revm::primitives::{CfgEnvWithHandlerCfg, SpecId};
use serde::{Deserialize, Serialize};
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

pub struct MultiEvmEnv<D, H>(pub HashMap<ExecutionLocation, EvmEnv<D, H>>);

impl<D, H> MultiEvmEnv<D, H> {
    pub fn from_single(env: EvmEnv<D, H>, location: ExecutionLocation) -> Self {
        let mut envs = HashMap::new();
        envs.insert(location, env);
        Self(envs)
    }
}

impl<D, H: EvmBlockHeader> MultiEvmEnv<D, H> {
    pub fn insert(&mut self, location: ExecutionLocation, env: EvmEnv<D, H>) {
        self.0.insert(location, env);
    }

    pub fn get_mut(
        &mut self,
        location: &ExecutionLocation,
    ) -> Result<&mut EvmEnv<D, H>, EngineError> {
        self.0
            .get_mut(location)
            .ok_or(EngineError::EvmNotFound(*location))
    }

    pub fn with_chain_spec(self, chain_spec: &ChainSpec) -> Result<Self, EngineError> {
        let envs = self
            .0
            .into_iter()
            .map(|(location, env)| Ok((location, env.with_chain_spec(chain_spec)?)))
            .collect::<Result<_, _>>()?;

        Ok(Self(envs))
    }
}

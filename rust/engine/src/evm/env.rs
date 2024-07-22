use alloy_primitives::{BlockNumber, ChainId};
use revm::primitives::{CfgEnvWithHandlerCfg, SpecId};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

use crate::{block_header::EvmBlockHeader, chain::spec::ChainSpec, engine::EngineError};

/// The environment to execute the contract calls in.
pub struct EvmEnv<D> {
    pub db: D,
    pub cfg_env: CfgEnvWithHandlerCfg,
    pub header: Box<dyn EvmBlockHeader>,
}

impl<D> EvmEnv<D> {
    /// Creates a new environment.
    /// It uses the default configuration for the latest specification.
    pub fn new(db: D, header: Box<dyn EvmBlockHeader>) -> Self {
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
    pub fn header(&self) -> &dyn EvmBlockHeader {
        self.header.as_ref()
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

pub type MultiEvmEnv<D> = HashMap<ExecutionLocation, EvmEnv<D>>;

use block_header::EvmBlockHeader;
use chain::ChainSpec;
use location::ExecutionLocation;
use revm::{
    primitives::{CfgEnvWithHandlerCfg, SpecId},
    DatabaseRef,
};

use crate::engine::EngineError;

pub mod cached;
pub mod location;

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

pub trait EvmEnvFactory<D>: Send + Sync
where
    D: DatabaseRef,
{
    fn create(&self, location: ExecutionLocation) -> anyhow::Result<EvmEnv<D>>;
}

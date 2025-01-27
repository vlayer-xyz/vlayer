use std::collections::HashMap;

use alloy_primitives::{BlockHash, BlockNumber, ChainId};
use block_header::EvmBlockHeader;
use chain::{ChainSpec, Error};
use derive_more::{Deref, DerefMut, From, Into, IntoIterator};
use revm::{
    primitives::{CfgEnvWithHandlerCfg, HandlerCfg, SpecId},
    DatabaseRef,
};

pub mod cached;
pub mod factory;
pub mod location;

/// The environment to execute the contract calls in.
pub struct EvmEnv<D: DatabaseRef + Send + Sync> {
    pub db: D,
    pub cfg_env: CfgEnvWithHandlerCfg,
    pub header: Box<dyn EvmBlockHeader>,
}

impl<D> EvmEnv<D>
where
    D: DatabaseRef + Send + Sync,
{
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
    pub fn with_chain_spec(mut self, chain_spec: &ChainSpec) -> Result<Self, Error> {
        self.cfg_env.chain_id = chain_spec.id();
        let spec_id = chain_spec.active_fork(self.header.number(), self.header.timestamp())?;
        let handler_cfg = HandlerCfg::new_with_optimism(spec_id, chain_spec.is_optimism());
        self.cfg_env.handler_cfg = handler_cfg;

        Ok(self)
    }

    /// Returns the header of the environment.
    pub fn header(&self) -> &dyn EvmBlockHeader {
        self.header.as_ref()
    }
}

#[derive(Debug, Clone, From, Deref, DerefMut, IntoIterator, Into)]
pub struct BlocksByChain(HashMap<ChainId, Vec<(BlockNumber, BlockHash)>>);

impl BlocksByChain {
    pub fn chain_ids(&self) -> Box<[ChainId]> {
        self.0.keys().cloned().collect()
    }
}

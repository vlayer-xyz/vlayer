use alloy_primitives::Sealed;
use revm::primitives::{CfgEnvWithHandlerCfg, SpecId};

use crate::chain::spec::ChainSpec;

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
    pub fn with_chain_spec(mut self, chain_spec: &ChainSpec) -> anyhow::Result<Self> {
        self.cfg_env.chain_id = chain_spec.chain_id();
        self.cfg_env.handler_cfg.spec_id =
            chain_spec.active_fork(self.header.number(), self.header.timestamp())?;
        Ok(self)
    }

    /// Returns the header of the environment.
    pub fn header(&self) -> &H {
        self.header.inner()
    }
}

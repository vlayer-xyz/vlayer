use block_header::EvmBlockHeader;
use chain::{ChainSpec, Error, OptimismSpec};
use revm::primitives::{CfgEnvWithHandlerCfg, HandlerCfg, SpecId};

pub mod cached;
pub mod factory;
pub mod location;

/// The environment to execute the contract calls in.
pub struct EvmEnv<D> {
    pub db: D,
    pub cfg_env: CfgEnvWithHandlerCfg,
    pub header: Box<dyn EvmBlockHeader>,
    pub optimism: Option<OptimismSpec>,
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
            optimism: None,
        }
    }

    /// Sets the chain ID and specification ID from the given chain spec.
    pub fn with_chain_spec(mut self, chain_spec: &ChainSpec) -> Result<Self, Error> {
        self.cfg_env.chain_id = chain_spec.id();
        let spec_id = chain_spec.active_fork(self.header.number(), self.header.timestamp())?;
        let handler_cfg = HandlerCfg::new_with_optimism(spec_id, chain_spec.is_optimism());
        self.cfg_env.handler_cfg = handler_cfg;
        self.optimism = chain_spec.optimism_spec();

        Ok(self)
    }

    /// Returns the header of the environment.
    pub fn header(&self) -> &dyn EvmBlockHeader {
        self.header.as_ref()
    }

    pub fn is_committing_to_chain(&self, chain_id: u64) -> bool {
        self.optimism
            .as_ref()
            .is_some_and(|optimism| optimism.parent_chain() == chain_id)
    }
}

#[cfg(test)]
mod emv_env {
    use alloy_primitives::ChainId;
    use block_header::EthBlockHeader;

    use super::*;

    const ETH_MAINNET_ID: ChainId = 1;
    const OPTIMISM_ID: ChainId = 10;
    const ETH_SEPOLIA_ID: ChainId = 11_155_111;

    fn setup_env(chain_id: ChainId) -> EvmEnv<()> {
        let header = Box::new(EthBlockHeader::default());
        EvmEnv::new((), header)
            .with_chain_spec(&chain_id.try_into().unwrap())
            .unwrap()
    }

    #[test]
    fn optimism_commits_to_mainnet() {
        let env = setup_env(OPTIMISM_ID);
        assert!(env.is_committing_to_chain(ETH_MAINNET_ID));
    }

    #[test]
    fn sepolia_doesnt_commit_to_mainnet() {
        let env = setup_env(ETH_SEPOLIA_ID);
        assert!(!env.is_committing_to_chain(ETH_MAINNET_ID));
    }
}

use alloy_primitives::ChainId;
use thiserror::Error;
use tracing::info;

use crate::{
    ChainSpec as BaseChainSpec, ConversionError as BaseConversionError,
    spec::AnchorStateRegistrySpec,
};

#[derive(Debug, Clone)]
pub struct ChainSpec {
    pub chain_spec: BaseChainSpec,
    pub anchor_chain: ChainId,
    pub anchor_state_registry: AnchorStateRegistrySpec,
}

#[derive(Debug, Error, PartialEq, Eq)]
pub enum CommitError {
    #[error("{src} chain does not commit into {dest} chain but into {anchor}")]
    WrongAnchorChain {
        src: ChainId,
        dest: ChainId,
        anchor: ChainId,
    },
}

impl ChainSpec {
    pub fn assert_anchor(&self, chain_id: ChainId) -> Result<(), CommitError> {
        if self.anchor_chain != chain_id {
            return Err(CommitError::WrongAnchorChain {
                src: chain_id,
                dest: self.chain_spec.id(),
                anchor: self.anchor_chain,
            });
        };
        info!("Chain {} commits into {}", self.chain_spec.id(), self.anchor_chain);
        Ok(())
    }
}

#[cfg(test)]
mod assert_commits_into {
    use super::*;

    const OP_MAINNET: ChainId = 10;
    const ETHEREUM_MAINNET: ChainId = 1;
    const ETHEREUM_SEPOLIA: ChainId = 11_155_111;

    #[test]
    fn optimism_mainnet_commits_to_eth_mainnet() -> anyhow::Result<()> {
        let spec = ChainSpec::try_from(OP_MAINNET)?;
        spec.assert_anchor(ETHEREUM_MAINNET)?;
        Ok(())
    }

    #[test]
    fn optimism_mainnet_doesnt_commit_to_eth_sepolia() -> anyhow::Result<()> {
        let spec = ChainSpec::try_from(OP_MAINNET)?;
        let result = spec.assert_anchor(ETHEREUM_SEPOLIA);

        assert!(matches!(
            result,
            Err(CommitError::WrongAnchorChain {
                src: ETHEREUM_SEPOLIA,
                dest: OP_MAINNET,
                anchor: ETHEREUM_MAINNET
            })
        ));
        Ok(())
    }
}

#[derive(Debug, Error, PartialEq, Eq)]
pub enum ConversionError {
    #[error("Conversion: {0}")]
    ConversionError(#[from] BaseConversionError),
    #[error("NotAnoptimism: {0}")]
    NotAnOptimism(ChainId),
}

impl TryFrom<ChainId> for ChainSpec {
    type Error = ConversionError;

    fn try_from(value: ChainId) -> Result<Self, Self::Error> {
        let chain_spec = BaseChainSpec::try_from(value)?;
        let op_spec = chain_spec
            .op_spec()
            .ok_or(ConversionError::NotAnOptimism(value))?;
        Ok(ChainSpec {
            chain_spec,
            anchor_chain: op_spec.anchor_chain(),
            anchor_state_registry: op_spec.anchor_state_registry(),
        })
    }
}

use alloy_primitives::{Address, BlockNumber, ChainId};
use revm::primitives::SpecId;
use serde::{Deserialize, Serialize};
use thiserror::Error;

use crate::{config::CHAIN_ID_TO_CHAIN_SPEC, fork::Fork};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChainSpec {
    id: ChainId,
    name: String,
    forks: Box<[Fork]>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    op_spec: Option<OptimismSpec>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OptimismSpec {
    anchor_chain: ChainId,
    anchor_state_registry: Address,
}

impl OptimismSpec {
    pub const fn anchor_chain(&self) -> ChainId {
        self.anchor_chain
    }

    pub const fn anchor_state_registry(&self) -> Address {
        self.anchor_state_registry
    }
}

#[derive(Debug, Error)]
pub enum Error {
    #[error("Unsupported fork for block {0}")]
    UnsupportedForkForBlock(BlockNumber),
    #[error("Unsupported chain id: {0}")]
    UnsupportedChainId(ChainId),
}

impl ChainSpec {
    pub fn new<F>(
        id: ChainId,
        name: impl Into<String>,
        forks: impl IntoIterator<Item = F>,
        op_spec: Option<OptimismSpec>,
    ) -> Self
    where
        F: Into<Fork>,
    {
        let name = name.into();
        let forks: Box<[Fork]> = forks.into_iter().map(Into::into).collect();
        assert!(!forks.is_empty(), "chain spec must have at least one fork");
        assert!(
            forks.windows(2).all(|w| w[0] < w[1]),
            "forks must be ordered by their activation conditions in ascending order",
        );

        ChainSpec {
            id,
            name,
            forks,
            op_spec,
        }
    }

    /// Returns the [SpecId] for a given block number and timestamp or an error if not supported.
    pub fn active_fork(&self, block_number: BlockNumber, timestamp: u64) -> Result<SpecId, Error> {
        for fork in self.forks.iter().rev() {
            if fork.active(block_number, timestamp) {
                return Ok(**fork);
            }
        }
        Err(Error::UnsupportedForkForBlock(block_number))
    }

    pub const fn id(&self) -> ChainId {
        self.id
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn op_spec(&self) -> Option<OptimismSpec> {
        self.op_spec.clone()
    }

    pub const fn is_optimism(&self) -> bool {
        self.op_spec.is_some()
    }
}

impl TryFrom<ChainId> for ChainSpec {
    type Error = Error;

    fn try_from(chain_id: ChainId) -> Result<Self, Self::Error> {
        let chain_spec = CHAIN_ID_TO_CHAIN_SPEC
            .get(&chain_id)
            .ok_or(Error::UnsupportedChainId(chain_id))?;
        Ok((*chain_spec).clone())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    mod new {
        use super::*;
        use crate::config::MAINNET_MERGE_BLOCK_TIMESTAMP;

        #[test]
        #[should_panic(expected = "chain spec must have at least one fork")]
        fn panics_if_no_forks() {
            ChainSpec::new(1, "", [] as [Fork; 0], None);
        }

        #[test]
        #[should_panic(
            expected = "forks must be ordered by their activation conditions in ascending order"
        )]
        fn forks_should_be_ordered_by_activation() {
            ChainSpec::new(
                1,
                "",
                [
                    Fork::after_timestamp(SpecId::MERGE, MAINNET_MERGE_BLOCK_TIMESTAMP),
                    Fork::after_block(SpecId::SHANGHAI, 0),
                ],
                None,
            );
        }

        #[test]
        fn success() {
            ChainSpec::new(
                1,
                "",
                [
                    Fork::after_block(SpecId::MERGE, 0),
                    Fork::after_timestamp(SpecId::SHANGHAI, MAINNET_MERGE_BLOCK_TIMESTAMP),
                ],
                None,
            );
        }
    }

    mod active_fork {
        use lazy_static::lazy_static;

        use super::*;
        use crate::MAINNET_MERGE_BLOCK_NUMBER;

        lazy_static! {
            static ref ETHEREUM_MAINNET: ChainSpec = ChainSpec::try_from(1).unwrap();
        }

        #[test]
        fn frontier_at_genesis() -> anyhow::Result<()> {
            let spec_id = ETHEREUM_MAINNET.active_fork(0, 0)?;
            assert_eq!(spec_id, SpecId::FRONTIER);

            Ok(())
        }

        #[test]
        fn merge_block() -> anyhow::Result<()> {
            let spec_id = ETHEREUM_MAINNET.active_fork(MAINNET_MERGE_BLOCK_NUMBER, 0)?;
            assert_eq!(spec_id, SpecId::MERGE);

            Ok(())
        }

        #[test]
        fn cancun_at_latest() -> anyhow::Result<()> {
            let spec_id = ETHEREUM_MAINNET.active_fork(0, u64::MAX)?;
            assert_eq!(spec_id, SpecId::CANCUN);

            Ok(())
        }
    }
}

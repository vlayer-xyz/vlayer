use std::ops::Deref;

use alloy_primitives::{BlockNumber, ChainId};
use anyhow::bail;
use revm::primitives::SpecId;
use serde::{Deserialize, Serialize};

use crate::{config::CHAIN_ID_TO_CHAIN_SPEC, error::ChainError, fork::Fork};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChainSpec {
    chain_id: ChainId,
    forks: Box<[Fork]>,
}

impl ChainSpec {
    pub fn new<F>(chain_id: ChainId, forks: impl IntoIterator<Item = F>) -> Self
    where
        F: Into<Fork>,
    {
        let forks: Box<[Fork]> = forks.into_iter().map(Into::into).collect();
        assert!(!forks.is_empty(), "chain spec must have at least one fork");
        assert!(
            forks.windows(2).all(|w| w[0] < w[1]),
            "forks must be ordered by their activation conditions in ascending order",
        );

        ChainSpec { chain_id, forks }
    }

    /// Creates a new configuration consisting of only one specification ID.
    pub fn new_single(chain_id: ChainId, spec_id: SpecId) -> Self {
        ChainSpec::new(chain_id, [Fork::after_block(spec_id, 0)])
    }

    /// Returns the [SpecId] for a given block number and timestamp or an error if not supported.
    pub fn active_fork(&self, block_number: BlockNumber, timestamp: u64) -> anyhow::Result<SpecId> {
        for fork in self.forks.iter().rev() {
            if fork.active(block_number, timestamp) {
                return Ok(**fork);
            }
        }
        bail!("unsupported fork for block {}", block_number)
    }
}

impl Deref for ChainSpec {
    type Target = ChainId;

    fn deref(&self) -> &Self::Target {
        &self.chain_id
    }
}

impl TryFrom<ChainId> for ChainSpec {
    type Error = ChainError;

    fn try_from(chain_id: ChainId) -> Result<Self, Self::Error> {
        let chain_spec = CHAIN_ID_TO_CHAIN_SPEC
            .get(&chain_id)
            .ok_or(ChainError::UnsupportedChainId(chain_id))?;
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
            ChainSpec::new(1, [] as [Fork; 0]);
        }

        #[test]
        #[should_panic(
            expected = "forks must be ordered by their activation conditions in ascending order"
        )]
        fn forks_should_be_ordered_by_activation() {
            ChainSpec::new(
                1,
                [
                    Fork::after_timestamp(SpecId::MERGE, MAINNET_MERGE_BLOCK_TIMESTAMP),
                    Fork::after_block(SpecId::SHANGHAI, 0),
                ],
            );
        }

        #[test]
        fn success() {
            ChainSpec::new(
                1,
                [
                    Fork::after_block(SpecId::MERGE, 0),
                    Fork::after_timestamp(SpecId::SHANGHAI, MAINNET_MERGE_BLOCK_TIMESTAMP),
                ],
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

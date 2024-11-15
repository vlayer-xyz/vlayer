use alloy_primitives::{BlockNumber, ChainId};
use anyhow::bail;
use revm::primitives::SpecId;
use serde::{Deserialize, Serialize};

use crate::{
    config::CHAIN_MAP,
    error::ChainError,
    fork::{after_block, Fork},
};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChainSpec {
    pub chain_id: ChainId,
    forks: Vec<Fork>,
}

impl ChainSpec {
    pub fn new<F>(chain_id: ChainId, forks: impl IntoIterator<Item = F>) -> Self
    where
        F: Into<Fork>,
    {
        let forks: Vec<Fork> = forks.into_iter().map(Into::into).collect();
        assert_ne!(forks.len(), 0, "chain spec must have at least one fork");
        assert!(
            forks.windows(2).all(|w| w[0] < w[1]),
            "forks must be ordered by their activation conditions in ascending order",
        );

        ChainSpec { chain_id, forks }
    }

    /// Creates a new configuration consisting of only one specification ID.
    pub fn new_single(chain_id: ChainId, spec_id: SpecId) -> Self {
        ChainSpec::new(chain_id, [after_block(spec_id, 0)])
    }

    /// Returns the [SpecId] for a given block number and timestamp or an error if not supported.
    pub fn active_fork(&self, block_number: BlockNumber, timestamp: u64) -> anyhow::Result<SpecId> {
        for fork in self.forks.iter().rev() {
            if fork.active(block_number, timestamp) {
                return Ok(fork.spec);
            }
        }
        bail!("unsupported fork for block {}", block_number)
    }
}

impl TryFrom<ChainId> for ChainSpec {
    type Error = ChainError;

    fn try_from(chain_id: ChainId) -> Result<Self, Self::Error> {
        let chain_spec = CHAIN_MAP
            .get(&chain_id)
            .ok_or(ChainError::UnsupportedChainId(chain_id))?;
        Ok((**chain_spec).clone())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    mod new {
        use super::*;
        use crate::{config::MAINNET_MERGE_BLOCK_TIMESTAMP, fork::after_timestamp};

        #[test]
        #[should_panic(expected = "chain spec must have at least one fork")]
        fn panics_if_no_forks() {
            let empty_forks: Vec<Fork> = vec![];
            ChainSpec::new(1, empty_forks);
        }

        #[test]
        #[should_panic(
            expected = "forks must be ordered by their activation conditions in ascending order"
        )]
        fn forks_should_be_ordered_by_activation() {
            ChainSpec::new(
                1,
                [
                    after_timestamp(SpecId::MERGE, MAINNET_MERGE_BLOCK_TIMESTAMP),
                    after_block(SpecId::SHANGHAI, 0),
                ],
            );
        }

        #[test]
        fn success() {
            ChainSpec::new(
                1,
                [
                    after_block(SpecId::MERGE, 0),
                    after_timestamp(SpecId::SHANGHAI, MAINNET_MERGE_BLOCK_TIMESTAMP),
                ],
            );
        }
    }
}

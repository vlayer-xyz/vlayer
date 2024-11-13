use std::collections::BTreeMap;

use alloy_primitives::{BlockNumber, ChainId};
use anyhow::bail;
use derive_new::new;
use revm::primitives::SpecId;
use serde::{Deserialize, Serialize};

use crate::{config::CHAIN_MAP, error::ChainError, fork::ForkCondition};

#[derive(Debug, Clone, Serialize, Deserialize, new)]
pub struct ChainSpec {
    pub chain_id: ChainId,
    max_spec_id: SpecId,
    forks: BTreeMap<SpecId, ForkCondition>,
}

impl ChainSpec {
    /// Creates a new configuration consisting of only one specification ID.
    pub fn new_single(chain_id: ChainId, spec_id: SpecId) -> Self {
        ChainSpec {
            chain_id,
            max_spec_id: spec_id,
            forks: BTreeMap::from([(spec_id, ForkCondition::Block(0))]),
        }
    }

    /// Returns the [SpecId] for a given block number and timestamp or an error if not supported.
    pub fn active_fork(&self, block_number: BlockNumber, timestamp: u64) -> anyhow::Result<SpecId> {
        for (spec_id, fork) in self.forks.iter().rev() {
            if fork.active(block_number, timestamp) {
                return Ok(*spec_id);
            }
        }
        bail!("no supported fork for block {}", block_number)
    }

    pub fn spec_id(&self, block_number: BlockNumber, timestamp: u64) -> Option<SpecId> {
        for (spec_id, fork) in self.forks.iter().rev() {
            if fork.active(block_number, timestamp) {
                return Some(*spec_id);
            }
        }
        None
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

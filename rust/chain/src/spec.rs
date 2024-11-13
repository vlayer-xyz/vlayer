use std::collections::BTreeMap;

use alloy_primitives::{BlockNumber, ChainId};
use anyhow::bail;
use derive_new::new;
use revm::primitives::SpecId;
use serde::{Deserialize, Serialize};

use crate::{config::CHAIN_MAP, error::ChainError};

#[derive(Debug, Clone, Serialize, Deserialize, new)]
pub struct ChainSpec {
    pub chain_id: ChainId,
    forks: BTreeMap<SpecId, ActivationCondition>,
}

impl ChainSpec {
    /// Creates a new configuration consisting of only one specification ID.
    pub fn new_single(chain_id: ChainId, spec_id: SpecId) -> Self {
        ChainSpec {
            chain_id,
            forks: BTreeMap::from([(spec_id, ActivationCondition::Block(0))]),
        }
    }

    /// Returns the [SpecId] for a given block number and timestamp or an error if not supported.
    pub fn active_fork(&self, block_number: BlockNumber, timestamp: u64) -> anyhow::Result<SpecId> {
        for (spec_id, fork) in self.forks.iter().rev() {
            if fork.active(block_number, timestamp) {
                return Ok(*spec_id);
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

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ActivationCondition {
    Block(BlockNumber),
    Timestamp(u64),
}

impl ActivationCondition {
    pub fn active(&self, block_number: BlockNumber, timestamp: u64) -> bool {
        match self {
            ActivationCondition::Block(block) => *block <= block_number,
            ActivationCondition::Timestamp(ts) => *ts <= timestamp,
        }
    }
}

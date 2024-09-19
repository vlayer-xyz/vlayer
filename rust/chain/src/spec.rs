use std::collections::BTreeMap;

use alloy_primitives::{BlockNumber, ChainId};
use anyhow::{bail, Context};
use revm::primitives::SpecId;
use serde::{Deserialize, Serialize};

use crate::config::CHAIN_MAP;

use crate::{eip1559::Eip1559Constants, error::ChainError, fork::ForkCondition};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChainSpec {
    chain_id: ChainId,
    max_spec_id: SpecId,
    hard_forks: BTreeMap<SpecId, ForkCondition>,
    gas_constants: BTreeMap<SpecId, Eip1559Constants>,
}

impl ChainSpec {
    pub fn new(
        chain_id: ChainId,
        max_spec_id: SpecId,
        hard_forks: BTreeMap<SpecId, ForkCondition>,
        gas_constants: BTreeMap<SpecId, Eip1559Constants>,
    ) -> Self {
        ChainSpec {
            chain_id,
            max_spec_id,
            hard_forks,
            gas_constants,
        }
    }
    /// Creates a new configuration consisting of only one specification ID.
    pub fn new_single(
        chain_id: ChainId,
        spec_id: SpecId,
        eip_1559_constants: Eip1559Constants,
    ) -> Self {
        ChainSpec {
            chain_id,
            max_spec_id: spec_id,
            hard_forks: BTreeMap::from([(spec_id, ForkCondition::Block(0))]),
            gas_constants: BTreeMap::from([(spec_id, eip_1559_constants)]),
        }
    }

    pub fn chain_id(&self) -> ChainId {
        self.chain_id
    }

    pub fn validate_spec_id(&self, spec_id: SpecId) -> anyhow::Result<()> {
        let (min_spec_id, _) = self.hard_forks.first_key_value().context("no hard forks")?;
        if spec_id < *min_spec_id {
            bail!("expected >= {:?}, got {:?}", min_spec_id, spec_id);
        }
        if spec_id > self.max_spec_id {
            bail!("expected <= {:?}, got {:?}", self.max_spec_id, spec_id);
        }
        Ok(())
    }
    /// Returns the [SpecId] for a given block number and timestamp or an error if not
    /// supported.
    pub fn active_fork(&self, block_number: BlockNumber, timestamp: u64) -> anyhow::Result<SpecId> {
        match self.spec_id(block_number, timestamp) {
            Some(spec_id) => {
                if spec_id > self.max_spec_id {
                    bail!("expected <= {:?}, got {:?}", self.max_spec_id, spec_id);
                } else {
                    Ok(spec_id)
                }
            }
            None => bail!("no supported fork for block {}", block_number),
        }
    }
    /// Returns the Eip1559 constants for a given [SpecId].
    pub fn gas_constants(&self, spec_id: SpecId) -> Option<&Eip1559Constants> {
        self.gas_constants.range(..=spec_id).next_back().map(|(_, v)| v)
    }

    pub fn spec_id(&self, block_number: BlockNumber, timestamp: u64) -> Option<SpecId> {
        for (spec_id, fork) in self.hard_forks.iter().rev() {
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
        let chain_spec =
            CHAIN_MAP.get(&chain_id).ok_or(ChainError::UnsupportedChainId(chain_id))?;
        Ok((**chain_spec).clone())
    }
}

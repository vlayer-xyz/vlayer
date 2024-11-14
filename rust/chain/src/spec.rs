use std::{
    cmp::Ordering,
    collections::{BTreeMap, HashSet},
};

use alloy_primitives::{BlockNumber, ChainId};
use anyhow::bail;
use revm::primitives::SpecId;
use serde::{Deserialize, Serialize};

use crate::{config::CHAIN_MAP, error::ChainError};

#[derive(Debug, Clone, Serialize, Deserialize, derive_new::new)]
pub struct ChainSpec {
    pub chain_id: ChainId,
    forks: BTreeMap<SpecId, ActivationCondition>,
}

#[derive(Debug, PartialEq, Eq)]
struct Fork {
    spec: SpecId,
    activation: ActivationCondition,
}

impl Fork {
    fn cmp(&self, other: &Self) -> Ordering {
        match (&self.activation, &other.activation) {
            (ActivationCondition::Block(a), ActivationCondition::Block(b)) => a.cmp(&b),
            (ActivationCondition::Timestamp(a), ActivationCondition::Timestamp(b)) => a.cmp(&b),
            (ActivationCondition::Block(_), ActivationCondition::Timestamp(_)) => Ordering::Less,
            (ActivationCondition::Timestamp(_), ActivationCondition::Block(_)) => Ordering::Greater,
        }
    }
}

impl ChainSpec {
    pub fn new1(chain_id: ChainId, forks: BTreeMap<SpecId, ActivationCondition>) -> Self {
        assert_ne!(forks.len(), 0, "must have at least one fork");
        assert!(
            forks.values().cloned().collect::<HashSet<_>>().len() == forks.len(),
            "cannot have two forks with same activation condition"
        );
        ensure_ordered(&forks);

        ChainSpec { chain_id, forks }
    }

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

fn ensure_ordered(forks: &BTreeMap<SpecId, ActivationCondition>) {
    let mut timestamp_found = false;

    for activation in forks.values() {
        match activation {
            ActivationCondition::Timestamp(_) => {
                timestamp_found = true;
            }
            ActivationCondition::Block(_) if timestamp_found => {
                panic!(
                    "forks with block activation should go before forks with timestamp activation"
                );
            }
            _ => {}
        }
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

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Hash)]
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

#[cfg(test)]
mod tests {
    use super::*;

    mod new {
        use super::*;

        #[test]
        #[should_panic(expected = "must have at least one fork")]
        fn panics_if_no_forks() {
            ChainSpec::new1(1, BTreeMap::new());
        }

        #[test]
        #[should_panic(expected = "cannot have two forks with same activation condition")]
        fn cannot_have_two_forks_with_same_timestamp() {
            ChainSpec::new1(
                1,
                BTreeMap::from([
                    (SpecId::MERGE, ActivationCondition::Timestamp(0)),
                    (SpecId::SHANGHAI, ActivationCondition::Timestamp(0)),
                ]),
            );
        }

        #[test]
        #[should_panic(
            expected = "forks with block activation should go before forks with timestamp activation"
        )]
        fn block_activation_should_go_before_timestamp_activation() {
            ChainSpec::new1(
                1,
                BTreeMap::from([
                    (SpecId::MERGE, ActivationCondition::Timestamp(0)),
                    (SpecId::SHANGHAI, ActivationCondition::Block(0)),
                ]),
            );
        }

        // #[test]
        // #[should_panic(expected = "block-activated forks are not ordered")]
        // fn block_activated_forks_are_ordered() {
        //     ChainSpec::new1(
        //         1,
        //         BTreeMap::from([
        //             (SpecId::MERGE, ActivationCondition::Block(1)),
        //             (SpecId::SHANGHAI, ActivationCondition::Block(0)),
        //         ]),
        //     );
        // }
    }
}
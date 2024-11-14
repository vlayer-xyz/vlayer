use std::{cmp::Ordering, collections::HashSet};

use alloy_primitives::{BlockNumber, ChainId};
use anyhow::bail;
use derive_new::new;
use revm::primitives::SpecId;
use serde::{Deserialize, Serialize};

use crate::{config::CHAIN_MAP, error::ChainError};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChainSpec {
    pub chain_id: ChainId,
    forks: Vec<Fork>,
}

impl ChainSpec {
    pub fn new<V, F>(chain_id: ChainId, forks: V) -> Self
    where
        V: Into<Vec<F>>,
        F: Into<Fork>,
    {
        let forks: Vec<Fork> = forks.into().into_iter().map(|f| f.into()).collect();
        assert_ne!(forks.len(), 0, "chain spec must have at least one fork");
        assert!(
            no_duplicated_activations(&forks),
            "duplicate activation conditions among forks are not allowed",
        );
        assert!(
            is_ordered(&forks),
            "forks must be ordered by their activation conditions in ascending order",
        );
        assert!(
            no_timestamps_before_2022(&forks),
            "forks cannot have activation timestamp earlier than 2022-01-01"
        );

        ChainSpec { chain_id, forks }
    }

    /// Creates a new configuration consisting of only one specification ID.
    pub fn new_single(chain_id: ChainId, spec_id: SpecId) -> Self {
        ChainSpec {
            chain_id,
            forks: vec![Fork::new(spec_id, ActivationCondition::Block(0))],
        }
    }

    /// Returns the [SpecId] for a given block number and timestamp or an error if not supported.
    pub fn active_fork(&self, block_number: BlockNumber, timestamp: u64) -> anyhow::Result<SpecId> {
        for fork in self.forks.iter().rev() {
            if fork.activation.active(block_number, timestamp) {
                return Ok(fork.spec);
            }
        }
        bail!("unsupported fork for block {}", block_number)
    }
}

fn is_ordered(forks: &[Fork]) -> bool {
    let mut forks = forks.iter();
    let mut last = forks.next().unwrap();
    for fork in forks {
        if last.cmp(fork) != Ordering::Less {
            return false;
        }
        last = fork;
    }
    true
}

fn no_duplicated_activations(forks: &[Fork]) -> bool {
    let mut set = HashSet::new();
    for fork in forks {
        if !set.insert(fork.activation) {
            return false;
        }
    }
    true
}

fn no_timestamps_before_2022(forks: &[Fork]) -> bool {
    const MIN_TIMESTAMP: u64 = 1640995200; // 2022-01-01 00:00:00 UTC
    forks.iter().all(|fork| match fork.activation {
        ActivationCondition::Timestamp(ts) => ts >= MIN_TIMESTAMP,
        _ => true,
    })
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

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, new, Hash)]
pub struct Fork {
    spec: SpecId,
    activation: ActivationCondition,
}

impl Fork {
    fn cmp(&self, other: &Self) -> Ordering {
        match (&self.activation, &other.activation) {
            (ActivationCondition::Block(a), ActivationCondition::Block(b)) => a.cmp(b),
            (ActivationCondition::Timestamp(a), ActivationCondition::Timestamp(b)) => a.cmp(b),
            (ActivationCondition::Block(_), ActivationCondition::Timestamp(_)) => Ordering::Less,
            (ActivationCondition::Timestamp(_), ActivationCondition::Block(_)) => Ordering::Greater,
        }
    }
}

impl From<(SpecId, ActivationCondition)> for Fork {
    fn from(tuple: (SpecId, ActivationCondition)) -> Self {
        Fork {
            spec: tuple.0,
            activation: tuple.1,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Hash)]
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
        #[should_panic(expected = "chain spec must have at least one fork")]
        fn panics_if_no_forks() {
            let empty_forks: Vec<Fork> = vec![];
            ChainSpec::new(1, empty_forks);
        }

        #[test]
        #[should_panic(expected = "duplicate activation conditions among forks are not allowed")]
        fn duplicate_activations() {
            ChainSpec::new(
                1,
                [
                    (SpecId::MERGE, ActivationCondition::Block(0)),
                    (SpecId::SHANGHAI, ActivationCondition::Block(0)),
                ],
            );
        }

        #[test]
        #[should_panic(
            expected = "forks must be ordered by their activation conditions in ascending order"
        )]
        fn forks_should_be_ordered_by_activation() {
            ChainSpec::new(
                1,
                [
                    (SpecId::MERGE, ActivationCondition::Timestamp(0)),
                    (SpecId::SHANGHAI, ActivationCondition::Block(0)),
                ],
            );
        }

        #[test]
        #[should_panic(expected = "forks cannot have activation timestamp earlier than 2022-01-01")]
        fn fork_timestamp_older_than_2022() {
            ChainSpec::new(1, [(SpecId::MERGE, ActivationCondition::Timestamp(0))]);
        }

        #[test]
        fn success() {
            ChainSpec::new(
                1,
                [
                    (SpecId::MERGE, ActivationCondition::Block(0)),
                    (SpecId::SHANGHAI, ActivationCondition::Timestamp(1640995200)),
                ],
            );
        }
    }
}

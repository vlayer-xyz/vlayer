use std::{
    cmp::Ordering,
    collections::{BTreeMap, HashSet},
};

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
    new_forks: Vec<Fork>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, new, Hash)]
pub struct Fork {
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
    pub fn new1(
        chain_id: ChainId,
        forks: BTreeMap<SpecId, ActivationCondition>,
        new_forks: Vec<Fork>,
    ) -> Self {
        assert_ne!(new_forks.len(), 0, "must have at least one fork");
        assert!(
            no_duplicated_activations(&new_forks),
            "cannot have two forks with same activation condition",
        );
        assert!(is_ordered(&new_forks), "forks are not ordered",);

        ChainSpec {
            chain_id,
            forks,
            new_forks: Vec::new(),
        }
    }

    /// Creates a new configuration consisting of only one specification ID.
    pub fn new_single(chain_id: ChainId, spec_id: SpecId) -> Self {
        ChainSpec {
            chain_id,
            forks: BTreeMap::from([(spec_id, ActivationCondition::Block(0))]),
            new_forks: Vec::new(),
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

fn is_ordered(forks: &Vec<Fork>) -> bool {
    let mut iter = forks.iter();
    let mut last = iter.next().unwrap();
    for fork in iter {
        if last.cmp(fork) != Ordering::Less {
            return false;
        }
        last = fork;
    }
    true
}

fn no_duplicated_activations(forks: &Vec<Fork>) -> bool {
    let mut set = HashSet::new();
    for fork in forks {
        if !set.insert(fork.activation.clone()) {
            return false;
        }
    }
    true
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
            ChainSpec::new1(1, BTreeMap::new(), Vec::new());
        }

        #[test]
        #[should_panic(expected = "cannot have two forks with same activation condition")]
        fn cannot_have_two_forks_with_same_value() {
            let fork_1 = Fork::new(SpecId::MERGE, ActivationCondition::Timestamp(0));
            let fork_2 = Fork::new(SpecId::SHANGHAI, ActivationCondition::Timestamp(0));
            ChainSpec::new1(1, BTreeMap::new(), vec![fork_1, fork_2]);
        }

        #[test]
        #[should_panic(expected = "forks are not ordered")]
        fn block_activation_should_go_before_timestamp_activation() {
            let fork_1 = Fork::new(SpecId::MERGE, ActivationCondition::Block(0));
            let fork_2 = Fork::new(SpecId::SHANGHAI, ActivationCondition::Timestamp(0));
            ChainSpec::new1(1, BTreeMap::new(), vec![fork_2, fork_1]);
        }
    }
}

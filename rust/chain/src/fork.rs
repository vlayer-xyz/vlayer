use std::{cmp::Ordering, ops::Deref};

use ActivationCondition::*;
use alloy_primitives::BlockNumber;
use derive_new::new;
use revm::primitives::SpecId;
use serde::{Deserialize, Serialize};

use crate::config::MAINNET_MERGE_BLOCK_TIMESTAMP;

#[derive(Debug, Clone, Serialize, Deserialize, new)]
pub struct Fork {
    spec: SpecId, // Gets ignored when comparing forks
    activation: ActivationCondition,
}

impl Fork {
    pub const fn active(&self, block_number: BlockNumber, timestamp: u64) -> bool {
        self.activation.active(block_number, timestamp)
    }

    pub fn after_block(spec_id: SpecId, block_number: BlockNumber) -> Self {
        (spec_id, Block(block_number)).into()
    }

    pub fn after_timestamp(spec_id: SpecId, timestamp: u64) -> Self {
        assert!(
            timestamp >= MAINNET_MERGE_BLOCK_TIMESTAMP,
            "fork activation timestamp must be after Merge"
        );
        (spec_id, Timestamp(timestamp)).into()
    }
}

impl Deref for Fork {
    type Target = SpecId;

    fn deref(&self) -> &Self::Target {
        &self.spec
    }
}

impl PartialEq for Fork {
    fn eq(&self, other: &Self) -> bool {
        self.activation == other.activation
    }
}

impl Eq for Fork {}

impl PartialOrd for Fork {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Fork {
    fn cmp(&self, other: &Self) -> Ordering {
        self.activation.cmp(&other.activation)
    }
}

impl From<(SpecId, ActivationCondition)> for Fork {
    fn from((spec, activation): (SpecId, ActivationCondition)) -> Self {
        Fork { spec, activation }
    }
}

// Private by design. Use `after_block` or `after_timestamp` to create a `Fork`.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Hash, Ord, PartialOrd)]
enum ActivationCondition {
    Block(BlockNumber),
    Timestamp(u64),
}

impl ActivationCondition {
    pub const fn active(&self, block_number: BlockNumber, timestamp: u64) -> bool {
        match self {
            Block(block) => *block <= block_number,
            Timestamp(ts) => *ts <= timestamp,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    mod fork_ord {

        use SpecId::*;

        use super::*;

        #[test]
        #[allow(clippy::nonminimal_bool)]
        fn ordered_by_activation() {
            let merge_0 = Fork::new(MERGE, Block(0));
            let merge_1 = Fork::new(MERGE, Block(1));
            let shanghai_0 = Fork::new(SHANGHAI, Block(0));
            let shanghai_1 = Fork::new(SHANGHAI, Block(1));

            assert!(merge_0 < merge_1);
            assert!(shanghai_0 < shanghai_1);
            assert!(merge_0 == shanghai_0);
            assert!(!(merge_0 < shanghai_0));
        }
    }

    mod after_timestamp {
        use super::*;

        #[test]
        #[should_panic(expected = "fork activation timestamp must be after Merge")]
        fn panics_if_timestamp_is_before_merge() {
            let timestamp = MAINNET_MERGE_BLOCK_TIMESTAMP - 1;
            Fork::after_timestamp(SpecId::MERGE, timestamp);
        }
    }
}

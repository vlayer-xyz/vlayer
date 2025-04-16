use std::cmp::min;

use alloy_primitives::BlockNumber;
use derivative::Derivative;
use derive_new::new;
use u64_range::{NonEmptyRange, Range};

const GENESIS: BlockNumber = 0;

#[derive(Clone, Derivative, Debug, new)]
pub struct PrependStrategy {
    max_back_propagation_blocks: u64,
}

impl PrependStrategy {
    #[allow(clippy::expect_used)]
    pub fn compute_prepend_range(&self, range: NonEmptyRange) -> (NonEmptyRange, Range) {
        let prepend_count = self.prepend_count(range);
        range.add_left(prepend_count).expect("Prepend overflow")
    }

    #[allow(clippy::unwrap_used)]
    fn prepend_count(&self, range: NonEmptyRange) -> u64 {
        if range.start() == GENESIS {
            return 0;
        }
        let range = NonEmptyRange::try_from_range(GENESIS..=range.start() - 1).unwrap(); // SAFETY: start > 0
        min(self.max_back_propagation_blocks, range.len())
    }
}

#[derive(Clone, Derivative, Debug, new)]
pub struct AppendStrategy {
    max_head_blocks: u64,
    confirmations: u64,
}

impl AppendStrategy {
    #[allow(clippy::expect_used)]
    pub fn compute_append_range(
        &self,
        range: NonEmptyRange,
        latest: BlockNumber,
    ) -> (NonEmptyRange, Range) {
        let append_count = self.append_count(range, latest);
        range.add_right(append_count).expect("Append overflow")
    }

    fn append_count(&self, range: NonEmptyRange, latest: BlockNumber) -> u64 {
        let pending = latest + 1; // Pending block has 0 confirmations
        let confirmed = (pending).saturating_sub(self.confirmations); // Genesis block is always confirmed
        let range: Range = (range.end() + 1..=confirmed).into();
        min(self.max_head_blocks, range.len())
    }
}

#[cfg(test)]
mod tests {
    use std::ops::RangeInclusive;

    use lazy_static::lazy_static;

    use super::*;

    const MAX_HEAD_BLOCKS: u64 = 10;
    const MAX_BACK_PROPAGATION_BLOCKS: u64 = 10;
    const CONFIRMATIONS: u64 = 2;

    lazy_static! {
        static ref PREPEND_STRATEGY: PrependStrategy =
            PrependStrategy::new(MAX_BACK_PROPAGATION_BLOCKS);
    }

    lazy_static! {
        static ref APPEND_STRATEGY: AppendStrategy =
            AppendStrategy::new(MAX_HEAD_BLOCKS, CONFIRMATIONS);
    }

    // Helper function to create a NonEmptyRange from RangeInclusive<u64>.
    // Panics if the range is empty.
    // It's named `r` to not clutter the tests:
    // assert_eq!(r(0..=1).trim_left(1), 1..=1) is more readable than assert_eq!(NonEmptyRange::try_from_range(0..=1).unwrap().trim_left(1), 1..=1)
    fn r(range: RangeInclusive<u64>) -> NonEmptyRange {
        NonEmptyRange::try_from_range(range).unwrap()
    }

    mod prepend {
        use super::*;

        #[test]
        #[allow(clippy::reversed_empty_ranges)]
        fn reached_genesis() {
            assert_eq!(PREPEND_STRATEGY.prepend_count(r(0..=0)), 0);
        }

        #[test]
        fn full_chunk() {
            assert_eq!(
                PREPEND_STRATEGY
                    .prepend_count(r(MAX_BACK_PROPAGATION_BLOCKS..=MAX_BACK_PROPAGATION_BLOCKS)),
                MAX_BACK_PROPAGATION_BLOCKS
            );
        }

        #[test]
        fn partial_chunk() {
            assert_eq!(
                PREPEND_STRATEGY.prepend_count(r(
                    MAX_BACK_PROPAGATION_BLOCKS - 1..=MAX_BACK_PROPAGATION_BLOCKS - 1
                )),
                MAX_BACK_PROPAGATION_BLOCKS - 1
            );
        }
    }

    mod append {
        use super::*;

        #[test]
        fn latest_is_genesis_does_not_cause_an_underflow_panic() {
            assert_eq!(APPEND_STRATEGY.append_count(r(0..=0), 0), 0);
        }

        #[test]
        fn same_block() {
            // Could happen after init or on 1-depth reorg
            assert_eq!(APPEND_STRATEGY.append_count(r(1..=1), 1), 0);
        }

        #[test]
        fn new_block_not_enough_confirmations() {
            assert_eq!(APPEND_STRATEGY.append_count(r(0..=0), 1), 0);
        }

        #[test]
        fn new_confirmed_block() {
            assert_eq!(APPEND_STRATEGY.append_count(r(0..=0), CONFIRMATIONS), 1);
        }

        #[test]
        fn many_new_confirmed_blocks() {
            assert_eq!(APPEND_STRATEGY.append_count(r(0..=0), 100), MAX_HEAD_BLOCKS);
        }
    }
}

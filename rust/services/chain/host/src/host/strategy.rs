use std::{
    cmp::min,
    fmt::{Display, Formatter},
};

use alloy_primitives::BlockNumber;
use derivative::Derivative;
use derive_new::new;
use tracing::info;
use u64_range::{NonEmptyRange, Range};

const GENESIS: BlockNumber = 0;

#[derive(Clone, Derivative, Debug, new)]
pub struct Strategy {
    max_head_blocks: u64,
    max_back_propagation_blocks: u64,
    confirmations: u64,
}

#[derive(Clone, Derivative, Debug, new)]
pub struct AppendStrategy {
    max_head_blocks: u64,
    confirmations: u64,
}

impl AppendStrategy {
    fn append_count(&self, range: NonEmptyRange, latest: BlockNumber) -> u64 {
        let confirmed = (latest + 1).saturating_sub(self.confirmations); // Genesis block is always confirmed
        let range: Range = (range.end() + 1..=confirmed).into();
        min(self.max_head_blocks, range.len())
    }

    pub fn range(&self, range: NonEmptyRange, latest: BlockNumber) -> Range {
        let append_count = self.append_count(range, latest);
        (range.end() + 1..=range.end() + append_count).into()
    }
}

#[derive(Clone, Derivative, Debug, new)]
pub struct PrependStrategy {
    max_back_propagation_blocks: u64,
}

impl PrependStrategy {
    fn prepend_count(&self, range: NonEmptyRange) -> u64 {
        if range.start() == GENESIS {
            return 0;
        }
        let range = NonEmptyRange::try_from_range(GENESIS..=range.start() - 1).unwrap(); // SAFETY: start > 0
        min(self.max_back_propagation_blocks, range.len())
    }

    pub fn range(&self, range: NonEmptyRange) -> Range {
        let prepend_count = self.prepend_count(range);
        (range.end() + 1..=range.end() + prepend_count).into()
    }
}

#[derive(Debug, Eq, PartialEq)]
pub struct AppendPrependRanges {
    pub prepend: Range,
    pub append: Range,
    pub new_range: NonEmptyRange,
}

impl AppendPrependRanges {
    pub fn new(old_range: NonEmptyRange, append_count: u64, prepend_count: u64) -> Self {
        let (new_range, append) = old_range.add_right(append_count).expect("Append overflow");
        let (new_range, prepend) = new_range.add_left(prepend_count).expect("Prepend overflow");
        Self {
            prepend,
            append,
            new_range,
        }
    }
}

impl Display for AppendPrependRanges {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "Prepend: {}, Range: {}, Append: {}",
            self.prepend, self.new_range, self.append
        )
    }
}

impl Strategy {
    // Tells Host which blocks to append and prepend.
    // The returned ranges are adjacent to the current range. [prepend][range][append]
    // Returned ranges can be empty.
    pub fn append_prepend_ranges(
        &self,
        range: NonEmptyRange,
        latest: BlockNumber,
    ) -> AppendPrependRanges {
        let ranges = AppendPrependRanges::new(
            range,
            self.get_append_count(range, latest),
            self.get_prepend_count(range),
        );
        info!("Append prepend ranges: {}", ranges);
        ranges
    }

    fn get_prepend_count(&self, range: NonEmptyRange) -> u64 {
        if range.start() == GENESIS {
            return 0;
        }
        let range = NonEmptyRange::try_from_range(GENESIS..=range.start() - 1).unwrap(); // SAFETY: start > 0
        min(self.max_back_propagation_blocks, range.len())
    }

    fn get_append_count(&self, range: NonEmptyRange, latest: BlockNumber) -> u64 {
        let confirmed = (latest + 1).saturating_sub(self.confirmations); // Genesis block is always confirmed
        let range: Range = (range.end() + 1..=confirmed).into();
        min(self.max_head_blocks, range.len())
    }
}

#[cfg(test)]
mod test {
    use std::ops::RangeInclusive;

    use lazy_static::lazy_static;

    use super::*;

    const MAX_HEAD_BLOCKS: u64 = 10;
    const MAX_BACK_PROPAGATION_BLOCKS: u64 = 10;
    const CONFIRMATIONS: u64 = 2;

    lazy_static! {
        static ref STRATEGY: Strategy =
            Strategy::new(MAX_HEAD_BLOCKS, MAX_BACK_PROPAGATION_BLOCKS, CONFIRMATIONS);
    }

    // Helper function to create a NonEmptyRange from RangeInclusive<u64>
    // Panics if the range is empty
    // It's named `r` to not clutter the tests
    // assert_eq!(r(0..=1).trim_left(1), 1..=1) is more readable than assert_eq!(NonEmptyRange::try_from_range(0..=1).unwrap().trim_left(1), 1..=1)
    fn r(range: RangeInclusive<u64>) -> NonEmptyRange {
        NonEmptyRange::try_from_range(range).unwrap()
    }

    mod append {

        use super::*;

        #[test]
        fn latest_is_genesis_does_not_cause_an_underflow_panic() {
            assert_eq!(STRATEGY.get_append_count(r(0..=0), 0), 0);
        }

        #[test]
        fn same_block() {
            // Could happen after init or on 1-depth reorg
            assert_eq!(STRATEGY.get_append_count(r(1..=1), 1), 0);
        }

        #[test]
        fn new_block_not_enough_confirmations() {
            assert_eq!(STRATEGY.get_append_count(r(0..=0), 1), 0);
        }

        #[test]
        fn new_confirmed_block() {
            assert_eq!(STRATEGY.get_append_count(r(0..=0), CONFIRMATIONS), 1);
        }

        #[test]
        fn many_new_confirmed_blocks() {
            assert_eq!(STRATEGY.get_append_count(r(0..=0), 100), MAX_HEAD_BLOCKS);
        }
    }

    mod prepend {
        use super::*;

        #[test]
        #[allow(clippy::reversed_empty_ranges)]
        fn reached_genesis() {
            assert_eq!(STRATEGY.get_prepend_count(r(0..=0)), 0);
        }

        #[test]
        fn full_chunk() {
            assert_eq!(
                STRATEGY.get_prepend_count(r(
                    MAX_BACK_PROPAGATION_BLOCKS..=MAX_BACK_PROPAGATION_BLOCKS
                )),
                MAX_BACK_PROPAGATION_BLOCKS
            );
        }

        #[test]
        fn partial_chunk() {
            assert_eq!(
                STRATEGY.get_prepend_count(r(
                    MAX_BACK_PROPAGATION_BLOCKS - 1..=MAX_BACK_PROPAGATION_BLOCKS - 1
                )),
                MAX_BACK_PROPAGATION_BLOCKS - 1
            );
        }
    }

    mod append_prepend_ranges {
        use super::*;

        #[test]
        fn success() {
            assert_eq!(
                STRATEGY.append_prepend_ranges(r(100..=100), 105),
                AppendPrependRanges {
                    prepend: (90..=99).into(),
                    append: (101..=104).into(),
                    new_range: r(90..=104)
                }
            );
        }
    }
}

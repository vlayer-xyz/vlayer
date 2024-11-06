use std::{
    cmp::{max, min},
    fmt::{Display, Formatter},
    ops::RangeInclusive,
};

use alloy_primitives::BlockNumber;
use derivative::Derivative;
use derive_new::new;
use tracing::info;

use super::range_utils::{limit_left, limit_right, EMPTY_RANGE};
use crate::host::range_utils::len;

const GENESIS: BlockNumber = 0;

#[derive(Clone, Derivative, Debug, new)]
pub struct Strategy {
    max_head_blocks: u64,
    max_back_propagation_blocks: u64,
    confirmations: u64,
}

#[derive(Debug, Eq, PartialEq)]
pub struct AppendPrependRanges {
    pub prepend: RangeInclusive<BlockNumber>,
    pub append: RangeInclusive<BlockNumber>,
    pub new_range: RangeInclusive<BlockNumber>,
}

impl AppendPrependRanges {
    pub fn new(
        old_range: &RangeInclusive<BlockNumber>,
        append: RangeInclusive<BlockNumber>,
        prepend: RangeInclusive<BlockNumber>,
    ) -> Self {
        let new_range =
            *min(prepend.start(), old_range.start())..=*max(append.end(), old_range.end());
        Self {
            prepend,
            append,
            new_range,
        }
    }
}

fn display_range_with_size(range: &RangeInclusive<BlockNumber>) -> String {
    format!("{:?} ({})", range, len(range))
}

impl Display for AppendPrependRanges {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(
            f,
            "Prepend: {}, Append: {}, New range: {}",
            display_range_with_size(&self.prepend),
            display_range_with_size(&self.append),
            display_range_with_size(&self.new_range)
        )
    }
}

impl Strategy {
    // Tells Host which blocks to append and prepend.
    // The returned ranges are adjacent to the current range. [prepend][range][append]
    // Returned ranges can be empty.
    pub fn get_append_prepend_ranges(
        &self,
        range: &RangeInclusive<BlockNumber>,
        latest: BlockNumber,
    ) -> AppendPrependRanges {
        let prepend = self.get_prepend_range(range);
        let append = self.get_append_range(range, latest);
        let ranges = AppendPrependRanges::new(range, append, prepend);
        info!("Append prepend ranges: {}", ranges);
        ranges
    }

    fn get_prepend_range(&self, range: &RangeInclusive<BlockNumber>) -> RangeInclusive<u64> {
        if *range.start() == GENESIS {
            return EMPTY_RANGE;
        }
        let range = 0..=range.start() - 1; // SAFETY: No underflow. Genesis is handled above
        limit_left(range, self.max_back_propagation_blocks)
    }

    fn get_append_range(
        &self,
        range: &RangeInclusive<BlockNumber>,
        latest: BlockNumber,
    ) -> RangeInclusive<u64> {
        let confirmed = (latest + 1).saturating_sub(self.confirmations); // Genesis block is always confirmed
        let range = range.end() + 1..=confirmed;
        limit_right(range, self.max_head_blocks)
    }
}

#[cfg(test)]
mod test {
    use lazy_static::lazy_static;

    use super::*;

    lazy_static! {
        static ref STRATEGY: Strategy = Strategy::new(10, 10, 2);
    }

    mod append {
        use super::*;

        #[test]
        #[allow(clippy::reversed_empty_ranges)]
        fn latest_is_genesis_no_underflow_panic() {
            assert_eq!(STRATEGY.get_append_range(&(0..=0), 0), 1..=0);
        }

        #[test]
        #[allow(clippy::reversed_empty_ranges)]
        fn same_block() {
            // Could happen after init or on 1-depth reorg
            assert_eq!(STRATEGY.get_append_range(&(1..=1), 1), 2..=0);
        }

        #[test]
        #[allow(clippy::reversed_empty_ranges)]
        fn new_block_not_enough_confirmations() {
            assert_eq!(STRATEGY.get_append_range(&(0..=0), 1), 1..=0);
        }

        #[test]
        fn new_confirmed_block() {
            assert_eq!(STRATEGY.get_append_range(&(0..=0), STRATEGY.confirmations), 1..=1);
        }

        #[test]
        fn many_new_confirmed_blocks() {
            assert_eq!(STRATEGY.get_append_range(&(0..=0), 100), 1..=STRATEGY.max_head_blocks);
        }
    }

    mod prepend {
        use super::*;

        #[test]
        #[allow(clippy::reversed_empty_ranges)]
        fn reached_genesis() {
            assert_eq!(STRATEGY.get_prepend_range(&(0..=0)), 1..=0);
        }

        #[test]
        fn full_chunk() {
            assert_eq!(
                STRATEGY.get_prepend_range(
                    &(STRATEGY.max_back_propagation_blocks..=STRATEGY.max_back_propagation_blocks)
                ),
                0..=STRATEGY.max_back_propagation_blocks - 1
            );
        }

        #[test]
        fn partial_chunk() {
            assert_eq!(
                STRATEGY.get_prepend_range(
                    &(STRATEGY.max_back_propagation_blocks - 1
                        ..=STRATEGY.max_back_propagation_blocks - 1)
                ),
                0..=STRATEGY.max_back_propagation_blocks - 2
            );
        }
    }

    mod append_prepend_ranges {
        use super::*;

        #[test]
        fn success() {
            assert_eq!(
                STRATEGY.get_append_prepend_ranges(&(100..=100), 105),
                AppendPrependRanges {
                    prepend: 90..=99,
                    append: 101..=104,
                    new_range: 90..=104
                }
            );
        }
    }
}

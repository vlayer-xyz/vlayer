use std::{
    cmp::{max, min},
    fmt::{Display, Formatter},
    ops::RangeInclusive,
};

use alloy_primitives::BlockNumber;
use derive_new::new;
use tracing::info;

use super::range_utils::{limit_left, limit_right};
use crate::host::range_utils::len;

pub const MAX_HEAD_BLOCKS: u64 = 10;
pub const MAX_BACK_PROPAGATION_BLOCKS: u64 = 10;
pub const CONFIRMATIONS: u64 = 2;
#[allow(clippy::reversed_empty_ranges)]
const EMPTY_RANGE: RangeInclusive<BlockNumber> = 1..=0;
const GENESIS: BlockNumber = 0;

#[derive(new)]
pub struct Strategy {
    #[new(value = "MAX_HEAD_BLOCKS")]
    max_head_blocks: u64,
    #[new(value = "MAX_BACK_PROPAGATION_BLOCKS")]
    max_back_propagation_blocks: u64,
    #[new(value = "CONFIRMATIONS")]
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
    // Tells Host - which blocks to append and prepend.
    // The ranges returned are touching the current range. [prepend][range][append]
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
    use super::*;

    mod append {
        use super::*;

        #[test]
        #[allow(clippy::reversed_empty_ranges)]
        fn latest_is_genesis_does_not_cause_an_underflow_panic() {
            assert_eq!(Strategy::new().get_append_range(&(0..=0), 0), 1..=0);
        }

        #[test]
        #[allow(clippy::reversed_empty_ranges)]
        fn same_block() {
            // Could happen after init or on 1-depth reorg
            assert_eq!(Strategy::new().get_append_range(&(1..=1), 1), 2..=0);
        }

        #[test]
        #[allow(clippy::reversed_empty_ranges)]
        fn new_block_not_enough_confirmations() {
            assert_eq!(Strategy::new().get_append_range(&(0..=0), 1), 1..=0);
        }

        #[test]
        fn new_confirmed_block() {
            assert_eq!(Strategy::new().get_append_range(&(0..=0), CONFIRMATIONS), 1..=1);
        }

        #[test]
        fn many_new_confirmed_blocks() {
            assert_eq!(Strategy::new().get_append_range(&(0..=0), 100), 1..=MAX_HEAD_BLOCKS);
        }
    }

    mod prepend {
        use super::*;

        #[test]
        #[allow(clippy::reversed_empty_ranges)]
        fn reached_genesis() {
            assert_eq!(Strategy::new().get_prepend_range(&(0..=0)), 1..=0);
        }

        #[test]
        fn full_chunk() {
            assert_eq!(
                Strategy::new().get_prepend_range(
                    &(MAX_BACK_PROPAGATION_BLOCKS..=MAX_BACK_PROPAGATION_BLOCKS)
                ),
                0..=MAX_BACK_PROPAGATION_BLOCKS - 1
            );
        }

        #[test]
        fn partial_chunk() {
            assert_eq!(
                Strategy::new().get_prepend_range(
                    &(MAX_BACK_PROPAGATION_BLOCKS - 1..=MAX_BACK_PROPAGATION_BLOCKS - 1)
                ),
                0..=MAX_BACK_PROPAGATION_BLOCKS - 2
            );
        }
    }

    mod append_prepend_ranges {
        use super::*;

        #[test]
        fn success() {
            assert_eq!(
                Strategy::new().get_append_prepend_ranges(&(100..=100), 105),
                AppendPrependRanges {
                    prepend: 90..=99,
                    append: 101..=104,
                    new_range: 90..=104
                }
            );
        }
    }
}

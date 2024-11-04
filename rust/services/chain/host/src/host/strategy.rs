use std::ops::RangeInclusive;

use super::range_utils::limit_right;

pub const MAX_HEAD_BLOCKS: u64 = 10;
pub const CONFIRMATIONS: u64 = 2;
const EMPTY_RANGE: RangeInclusive<u64> = 1..=0;

pub struct Strategy {}

impl Strategy {
    pub const fn new() -> Self {
        Strategy {}
    }

    pub fn get_append_prepend_ranges(
        &self,
        range: &RangeInclusive<u64>,
        latest: u64,
    ) -> (RangeInclusive<u64>, RangeInclusive<u64>) {
        (EMPTY_RANGE, self.get_append_range(range, latest))
    }

    fn get_append_range(&self, range: &RangeInclusive<u64>, latest: u64) -> RangeInclusive<u64> {
        if latest + 1 < CONFIRMATIONS {
            // With CONFIRMATIONS set at 2 - this only happens if latest == 0
            // Should not happen on any reasonable chain.
            return EMPTY_RANGE;
        }
        let confirmed = latest + 1 - CONFIRMATIONS;
        let range = range.end() + 1..=confirmed;
        limit_right(range, MAX_HEAD_BLOCKS)
    }
}

#[cfg(test)]
mod test {
    use super::*;

    mod append {
        use super::*;

        #[test]
        fn latest_is_genesis_does_not_cause_an_underflow_panic() {
            assert_eq!(Strategy::new().get_append_range(&(0..=0), 0), 1..=0);
        }

        #[test]
        fn same_block() {
            // Could happen after init or on 1-depth reorg
            assert_eq!(Strategy::new().get_append_range(&(1..=1), 1), 2..=0);
        }

        #[test]
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
}

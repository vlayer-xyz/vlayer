use std::{
    cmp::{max, min},
    ops::RangeInclusive,
};

#[allow(clippy::reversed_empty_ranges)]
pub const EMPTY_RANGE: RangeInclusive<u64> = 1..=0;

pub fn len(range: &RangeInclusive<u64>) -> u64 {
    if range.is_empty() {
        return 0;
    }
    assert_ne!(range, &(0..=u64::MAX), "Range length overflow");
    *range.end() - *range.start() + 1
}

pub fn limit_right(range: RangeInclusive<u64>, limit: u64) -> RangeInclusive<u64> {
    if range.is_empty() {
        return range;
    }
    if limit == 0 {
        return EMPTY_RANGE;
    }
    *range.start()..=min(*range.end(), (*range.start()).saturating_add(limit - 1))
}

pub fn limit_left(range: RangeInclusive<u64>, limit: u64) -> RangeInclusive<u64> {
    if range.is_empty() {
        return range;
    }
    if limit == 0 {
        return EMPTY_RANGE;
    }
    max((*range.end()).saturating_sub(limit - 1), *range.start())..=*range.end()
}

#[cfg(test)]
mod tests {
    use super::*;

    mod len {
        use super::*;

        #[test]
        #[allow(clippy::reversed_empty_ranges)]
        fn empty_range() {
            assert_eq!(len(&(1..=0)), 0)
        }

        #[test]
        #[should_panic(expected = "Range length overflow")]
        fn panics_on_len_overflow() {
            len(&(0..=u64::MAX));
        }

        #[test]
        fn non_empty_range() {
            assert_eq!(len(&(0..=0)), 1)
        }
    }

    mod limit_right {
        use super::*;

        #[test]
        #[allow(clippy::reversed_empty_ranges)]
        fn return_empty_range_when_limit_is_zero() {
            assert_eq!(limit_right(0..=0, 0), 1..=0);
        }

        #[test]
        fn overflow_does_not_panic_but_saturates() {
            assert_eq!(limit_right(u64::MAX..=u64::MAX, 100), u64::MAX..=u64::MAX);
        }

        #[test]
        #[allow(clippy::reversed_empty_ranges)]
        fn empty_range() {
            assert_eq!(limit_right(1..=0, 0), 1..=0)
        }

        #[test]
        fn range_smaller_than_limit() {
            assert_eq!(limit_right(0..=0, 2), 0..=0)
        }

        #[test]
        fn range_equal_to_limit() {
            assert_eq!(limit_right(0..=0, 1), 0..=0)
        }

        #[test]
        fn range_larger_than_limit() {
            assert_eq!(limit_right(0..=1, 1), 0..=0)
        }
    }

    mod limit_left {
        use super::*;

        #[test]
        #[allow(clippy::reversed_empty_ranges)]
        fn empty_range() {
            assert_eq!(limit_left(1..=0, 0), 1..=0)
        }

        #[test]
        fn range_smaller_than_limit() {
            assert_eq!(limit_left(0..=0, 2), 0..=0)
        }

        #[test]
        fn range_equal_to_limit() {
            assert_eq!(limit_left(0..=0, 1), 0..=0)
        }

        #[test]
        fn range_larger_than_limit() {
            assert_eq!(limit_left(0..=1, 1), 1..=1)
        }
    }
}

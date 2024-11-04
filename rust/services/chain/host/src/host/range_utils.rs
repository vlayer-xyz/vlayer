use std::{cmp::min, ops::RangeInclusive};

pub fn limit_right(range: RangeInclusive<u64>, limit: u64) -> RangeInclusive<u64> {
    if range.is_empty() {
        return range;
    }
    *range.start()..=min(*range.end(), *range.start() + limit - 1)
}

#[cfg(test)]
mod tests {
    use super::*;

    mod limit_right {
        use super::*;

        #[test]
        #[should_panic(expected = "attempt to subtract with overflow")]
        fn panics_when_limit_is_zero_and_range_starts_with_zero() {
            limit_right(0..=0, 0);
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
}

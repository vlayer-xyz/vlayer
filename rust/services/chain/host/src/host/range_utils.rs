use std::ops::RangeInclusive;

pub fn limit_right(range: RangeInclusive<u64>, limit: u64) -> RangeInclusive<u64> {
    if range.end() < range.start() {
        return range;
    }
    let size = range.end() - range.start() + 1;
    if size < limit {
        return range;
    }
    let end = range.start() + limit - 1;
    *range.start()..=end
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

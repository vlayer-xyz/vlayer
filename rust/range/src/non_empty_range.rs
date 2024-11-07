use std::{borrow::Borrow, fmt::Display, ops::RangeInclusive};

use crate::Range;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
/// Inclusive range that is guaranteed to be non-empty
pub struct NonEmptyRange {
    start: u64,
    end: u64,
}

impl NonEmptyRange {
    pub const fn from_single_value(value: u64) -> Self {
        Self {
            start: value,
            end: value,
        }
    }

    pub fn try_from_range(range: RangeInclusive<u64>) -> Option<Self> {
        (range.start() <= range.end()).then(|| Self {
            start: *range.start(),
            end: *range.end(),
        })
    }

    /// Panics if the range is empty
    fn from_range(range: RangeInclusive<u64>) -> Self {
        assert!(range.start() <= range.end());
        Self {
            start: *range.start(),
            end: *range.end(),
        }
    }

    pub const fn start(&self) -> u64 {
        self.start
    }

    pub const fn end(&self) -> u64 {
        self.end
    }

    pub fn len(&self) -> u64 {
        if self.end - self.start == u64::MAX {
            panic!("Range length overflow");
        }
        self.end - self.start + 1
    }

    pub const fn is_empty(&self) -> bool {
        false
    }

    pub fn add_right(&self, count: u64) -> Option<(NonEmptyRange, Range)> {
        if count == 0 {
            return Some((*self, Range::EMPTY));
        }
        let new_end = self.end.checked_add(count)?;
        // Extending non-empty range yields a non-empty range
        let new_range = Self::from_range(self.start..=new_end);
        let append = (self.end.checked_add(1)?..=new_end).into();
        Some((new_range, append))
    }

    pub fn add_left(&self, count: u64) -> Option<(NonEmptyRange, Range)> {
        if count == 0 {
            return Some((*self, Range::EMPTY));
        }
        let new_start = self.start.checked_sub(count)?;
        // Extending non-empty range yields a non-empty range
        let new_range = Self::from_range(new_start..=self.end);
        let prepend = (new_start..=self.start.checked_sub(1)?).into();
        Some((new_range, prepend))
    }

    pub const fn contains(&self, value: u64) -> bool {
        self.start <= value && value <= self.end
    }
}

impl From<u64> for NonEmptyRange {
    fn from(value: u64) -> Self {
        Self::from_single_value(value)
    }
}

impl From<NonEmptyRange> for RangeInclusive<u64> {
    fn from(range: NonEmptyRange) -> Self {
        range.start..=range.end
    }
}

impl From<&NonEmptyRange> for RangeInclusive<u64> {
    fn from(range: &NonEmptyRange) -> Self {
        range.start..=range.end
    }
}

impl<R: Borrow<RangeInclusive<u64>>> PartialEq<R> for NonEmptyRange {
    fn eq(&self, other: &R) -> bool {
        other.borrow() == &self.into()
    }
}

impl Display for NonEmptyRange {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "[{}..{}] ({})", self.start, self.end, self.len())
    }
}

impl IntoIterator for NonEmptyRange {
    type IntoIter = std::ops::Range<u64>;
    type Item = u64;

    fn into_iter(self) -> Self::IntoIter {
        self.start..self.end
    }
}

#[cfg(test)]
#[allow(clippy::reversed_empty_ranges)]
mod tests {

    use super::*;

    // Helper function to create a NonEmptyRange from RangeInclusive<u64>
    // Panics if the range is empty
    // It's named `r` to not clutter the tests
    // assert_eq!(r(0..=1).trim_left(1), 1..=1) is more readable than assert_eq!(NonEmptyRange::try_from_range(0..=1).unwrap().trim_left(1), 1..=1)
    fn r(range: RangeInclusive<u64>) -> NonEmptyRange {
        NonEmptyRange::try_from_range(range).unwrap()
    }

    #[test]
    fn try_from_range() {
        assert_eq!(NonEmptyRange::try_from_range(1..=1), Some(NonEmptyRange { start: 1, end: 1 }));
        assert_eq!(NonEmptyRange::try_from_range(1..=0), None);
    }

    mod len {
        use super::*;

        #[test]
        fn success() {
            assert_eq!(NonEmptyRange { start: 1, end: 1 }.len(), 1);
            assert_eq!(
                NonEmptyRange {
                    start: 0,
                    end: u64::MAX - 1
                }
                .len(),
                u64::MAX
            );
        }

        #[test]
        #[should_panic(expected = "Range length overflow")]
        fn overflow() {
            NonEmptyRange {
                start: 0,
                end: u64::MAX,
            }
            .len();
        }
    }

    #[test]
    fn is_empty() {
        assert!(!NonEmptyRange::from_single_value(1).is_empty());
    }

    #[test]
    fn add_right() {
        assert_eq!(r(0..=1).add_right(0), Some((r(0..=1), Range::EMPTY)));
        assert_eq!(r(0..=1).add_right(1), Some((r(0..=2), (2..=2).into())));
        assert_eq!(r(0..=1).add_right(2), Some((r(0..=3), (2..=3).into())));
        assert_eq!(r(0..=u64::MAX).add_right(1), None);
    }

    #[test]
    fn add_left() {
        assert_eq!(r(2..=3).add_left(0), Some((r(2..=3), Range::EMPTY)));
        assert_eq!(r(2..=3).add_left(1), Some((r(1..=3), (1..=1).into())));
        assert_eq!(r(2..=3).add_left(2), Some((r(0..=3), (0..=1).into())));
        assert_eq!(r(2..=3).add_left(3), None);
    }

    #[test]
    fn contains() {
        assert!(!r(1..=2).contains(0));
        assert!(r(1..=2).contains(1));
        assert!(r(1..=2).contains(2));
        assert!(!r(1..=2).contains(3));
    }
}

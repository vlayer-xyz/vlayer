use std::{borrow::Borrow, fmt::Display, ops::RangeInclusive};

use thiserror::Error;

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

    #[allow(clippy::panic)]
    pub fn len(&self) -> u64 {
        if self.end - self.start == u64::MAX {
            panic!("Range length overflow");
        }
        self.end - self.start + 1
    }

    pub const fn is_empty(&self) -> bool {
        false
    }

    /// Extends the range by adding the specified number of elements to the right.
    ///
    /// # Parameters
    /// - `count`: The number of elements to add to the right of the range.
    ///
    /// # Returns
    /// - `Some((NonEmptyRange, Range))` if the range can be successfully extended:
    ///   - `NonEmptyRange`: The extended range, which includes the original range plus `count` elements to the right.
    ///   - `Range`: A `Range` representing the newly added portion (i.e., the segment from `self.end + 1` to `new_end`).
    /// - `None` if the extension would cause an overflow or if `count` is such that the new end is invalid.
    ///
    /// # Edge Cases
    /// - If `count` is `0`, returns the original range and an empty appended range.
    /// - If adding `count` to `self.end` causes an overflow, returns `None`.
    pub fn add_right(&self, count: u64) -> Option<(NonEmptyRange, Range)> {
        if count == 0 {
            return Some((*self, Range::EMPTY));
        }
        let new_end = self.end.checked_add(count)?;
        // Extending non-empty range yields a non-empty range
        let new_range = Self::from_range(self.start..=new_end);
        let appended = (self.end.checked_add(1)?..=new_end).into();
        Some((new_range, appended))
    }

    /// Extends the range by adding the specified number of elements to the left.
    ///
    /// # Parameters
    /// - `count`: The number of elements to add to the left of the range.
    ///
    /// # Returns
    /// - `Some((NonEmptyRange, Range))` if the range can be successfully extended:
    ///   - `NonEmptyRange`: The extended range, which includes the original range plus `count` elements to the left.
    ///   - `Range`: A `Range` representing the newly added portion (i.e., the segment from `new_start` to `self.start - 1`).
    /// - `None` if the extension would cause an underflow or if `count` is such that the new start is invalid.
    ///
    /// # Edge Cases
    /// - If `count` is `0`, returns the original range and an empty prepended range.
    /// - If subtracting `count` from `self.start` causes an underflow, returns `None`.
    pub fn add_left(&self, count: u64) -> Option<(NonEmptyRange, Range)> {
        if count == 0 {
            return Some((*self, Range::EMPTY));
        }
        let new_start = self.start.checked_sub(count)?;
        // Extending non-empty range yields a non-empty range
        let new_range = Self::from_range(new_start..=self.end);
        let prepended = (new_start..=self.start.checked_sub(1)?).into();
        Some((new_range, prepended))
    }

    pub const fn contains(&self, value: u64) -> bool {
        self.start <= value && value <= self.end
    }

    pub fn find_ge<T: Ord + Copy, E, F>(&self, value: T, f: F) -> Result<Option<u64>, E>
    where
        F: Fn(u64) -> Result<T, E>,
    {
        if f(self.start)? >= value {
            return Ok(Some(self.start));
        }
        if f(self.end)? < value {
            return Ok(None);
        }
        // f(l) < value <= f(r)
        let mut l = self.start;
        let mut r = self.end;
        // while distance between l and r is greater than 1
        while r - l > 1 {
            let m = l + (r - l) / 2;
            let m_value = f(m)?;
            if m_value < value {
                // f(m) < value
                l = m;
            } else {
                // value <= f(m)
                r = m;
            }
        }
        Ok(Some(r))
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

#[derive(Debug, Error)]
pub enum Error {
    #[error("Range is empty")]
    Empty,
}

impl TryFrom<RangeInclusive<u64>> for NonEmptyRange {
    type Error = Error;

    fn try_from(range: RangeInclusive<u64>) -> Result<Self, Self::Error> {
        NonEmptyRange::try_from_range(range).ok_or(Error::Empty)
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

    mod lower_bound {

        use super::*;

        const fn identity(x: u64) -> Result<u64, ()> {
            Ok(x)
        }

        #[test]
        fn element_in_range() {
            assert_eq!(r(1..=1).find_ge(1, identity), Ok(Some(1)));
        }

        #[test]
        fn element_not_in_range() {
            assert_eq!(r(1..=1).find_ge(0, identity), Ok(Some(1)));
            assert_eq!(r(1..=1).find_ge(2, identity), Ok(None));
        }

        #[test]
        fn error() {
            assert_eq!(r(1..=1).find_ge(0, |_| Err(())), Err(()));
        }

        #[test]
        fn lower_bound() {
            let mul_2 = |x| Ok::<_, ()>(x * 2);
            assert_eq!(r(1..=2).find_ge(1, mul_2), Ok(Some(1)));
            assert_eq!(r(1..=2).find_ge(2, mul_2), Ok(Some(1)));
            assert_eq!(r(1..=2).find_ge(3, mul_2), Ok(Some(2)));
            assert_eq!(r(1..=2).find_ge(5, mul_2), Ok(None));

            assert_eq!(r(1..=100).find_ge(0, mul_2), Ok(Some(1)));
            assert_eq!(r(1..=100).find_ge(1, mul_2), Ok(Some(1)));
            assert_eq!(r(1..=100).find_ge(2, mul_2), Ok(Some(1)));
            assert_eq!(r(1..=100).find_ge(3, mul_2), Ok(Some(2)));
            assert_eq!(r(1..=100).find_ge(50, mul_2), Ok(Some(25)));
            assert_eq!(r(1..=100).find_ge(51, mul_2), Ok(Some(26)));
            assert_eq!(r(1..=100).find_ge(200, mul_2), Ok(Some(100)));
            assert_eq!(r(1..=100).find_ge(201, mul_2), Ok(None));
        }
    }
}

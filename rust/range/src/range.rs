use std::{fmt::Display, ops::RangeInclusive};

use derive_more::{From, Into};

use crate::NonEmptyRange;

#[derive(Debug, Copy, Clone, PartialEq, Eq, From, Into)]
pub struct Range(Option<NonEmptyRange>);

impl Range {
    pub const EMPTY: Range = Range(None);

    pub fn from_range(range: RangeInclusive<u64>) -> Self {
        NonEmptyRange::try_from_range(range).into()
    }

    pub const fn is_empty(&self) -> bool {
        self.0.is_none()
    }

    pub const fn as_non_empty(&self) -> Option<NonEmptyRange> {
        self.0
    }

    pub fn start(&self) -> Option<u64> {
        self.0.map(|range| range.start())
    }

    pub fn end(&self) -> Option<u64> {
        self.0.map(|range| range.end())
    }

    pub fn len(&self) -> u64 {
        self.0.map(|range| range.len()).unwrap_or(0)
    }

    pub fn trim_left(&self, limit: u64) -> Self {
        Self(self.0.and_then(|range| range.trim_left(limit).0))
    }

    pub fn trim_right(&self, limit: u64) -> Self {
        Self(self.0.and_then(|range| range.trim_right(limit).0))
    }
}

impl From<RangeInclusive<u64>> for Range {
    fn from(range: RangeInclusive<u64>) -> Self {
        NonEmptyRange::try_from_range(range).into()
    }
}

impl From<NonEmptyRange> for Range {
    fn from(range: NonEmptyRange) -> Self {
        Self(Some(range))
    }
}

impl PartialEq<RangeInclusive<u64>> for Range {
    fn eq(&self, other: &RangeInclusive<u64>) -> bool {
        self.0.map(|range| range == other).unwrap_or(false)
    }
}

impl Display for Range {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0.map_or("∅".to_string(), |range| range.to_string()))
    }
}

impl IntoIterator for Range {
    type IntoIter = std::ops::RangeInclusive<u64>;
    type Item = u64;

    fn into_iter(self) -> Self::IntoIter {
        #[allow(clippy::reversed_empty_ranges)]
        self.0.map(Into::into).unwrap_or(1..=0)
    }
}

#[cfg(test)]
#[allow(clippy::reversed_empty_ranges)]
mod tests {
    use super::*;

    #[test]
    fn from_range() {
        assert_eq!(Range::from_range(1..=1), Range(Some(NonEmptyRange::from_value(1))));
        assert_eq!(Range::from_range(1..=0), Range::EMPTY);
    }

    #[test]
    fn trim_left() {
        assert_eq!(Range::from_range(1..=2).trim_left(0), Range::EMPTY);
        assert_eq!(Range::from_range(1..=2).trim_left(1), Range::from_range(2..=2));
        assert_eq!(Range::from_range(1..=2).trim_left(2), Range::from_range(1..=2));
        assert_eq!(Range::from_range(1..=2).trim_left(3), Range::from_range(1..=2));
    }

    #[test]
    fn trim_right() {
        assert_eq!(Range::from_range(1..=2).trim_right(0), Range::EMPTY);
        assert_eq!(Range::from_range(1..=2).trim_right(1), Range::from_range(1..=1));
        assert_eq!(Range::from_range(1..=2).trim_right(2), Range::from_range(1..=2));
        assert_eq!(Range::from_range(1..=2).trim_right(3), Range::from_range(1..=2));
    }
}

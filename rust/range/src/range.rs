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
        self.0.map_or(0, |range| range.len())
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
        self.0.is_some_and(|range| range == other)
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
        self.0.map_or(1..=0, Into::into)
    }
}

#[cfg(test)]
#[allow(clippy::reversed_empty_ranges)]
mod tests {
    use super::*;

    #[test]
    fn is_empty() {
        assert!(Range::EMPTY.is_empty());
        assert!(!Range::from_range(1..=1).is_empty());
    }

    #[test]
    fn as_non_empty() {
        assert_eq!(Range::EMPTY.as_non_empty(), None);
        assert_eq!(
            Range::from_range(1..=1).as_non_empty(),
            Some(NonEmptyRange::from_single_value(1))
        );
    }

    #[test]
    fn len() {
        assert_eq!(Range::EMPTY.len(), 0);
        assert_eq!(Range::from_range(1..=1).len(), 1);
    }

    #[test]
    fn from_range() {
        assert_eq!(Range::from_range(1..=1), Range(Some(NonEmptyRange::from_single_value(1))));
        assert_eq!(Range::from_range(1..=0), Range::EMPTY);
    }
}

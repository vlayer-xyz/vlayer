use std::fmt::Display;

use derive_more::From;
use thousands::Separable;

#[derive(From)]
pub struct Count(u64);

impl Display for Count {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.0.separate_with_underscores().fmt(f)
    }
}

#[derive(From)]
pub struct Diff(i64);

impl Diff {
    pub fn new(actual: u64, snapshot: u64) -> Self {
        Self(i64::try_from(actual).unwrap() - i64::try_from(snapshot).unwrap())
    }
}

impl Display for Diff {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.0.separate_with_underscores().fmt(f)
    }
}

#[derive(From)]
pub struct PercentageDiff(f64);

impl PercentageDiff {
    pub const fn new(actual: u64, snapshot: u64) -> Self {
        // It's only for display. We don't care too much about the precision.
        #[allow(clippy::cast_possible_truncation, clippy::cast_precision_loss)]
        let percentage_diff = ((actual as f64 / snapshot as f64) * 100.0) - 100.0;
        Self(percentage_diff)
    }
}

impl Display for PercentageDiff {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:.2} %", self.0)
    }
}

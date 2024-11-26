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
        Self(actual as i64 - snapshot as i64)
    }
}

impl Display for Diff {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.0.separate_with_underscores().fmt(f)
    }
}

#[derive(From)]
pub struct PercentageDiff(i64);

impl PercentageDiff {
    pub fn new(actual: u64, snapshot: u64) -> Self {
        let percentage_diff = ((actual as f64 / snapshot as f64) * 100.0) as i64 - 100;
        Self(percentage_diff)
    }
}

impl Display for PercentageDiff {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}%", self.0)
    }
}

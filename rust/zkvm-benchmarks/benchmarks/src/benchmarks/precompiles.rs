use lazy_static::lazy_static;

use crate::{benchmarks::merge, Benchmark};
mod email;
mod url_pattern;
lazy_static! {
    pub static ref BENCHMARKS: Vec<Benchmark> = merge([
        ("email", email::BENCHMARKS.clone()),
        ("url_pattern", url_pattern::BENCHMARKS.clone()),
    ]);
}

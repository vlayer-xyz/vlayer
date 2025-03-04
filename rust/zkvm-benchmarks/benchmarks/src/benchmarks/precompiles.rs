use crate::{benchmarks::merge, Benchmark};
#[allow(dead_code)]
mod email;
mod url_pattern;

pub fn benchmarks() -> Vec<Benchmark> {
    merge([
        // ("email", email::benchmarks()),
        ("url_pattern", url_pattern::benchmarks()),
    ])
}

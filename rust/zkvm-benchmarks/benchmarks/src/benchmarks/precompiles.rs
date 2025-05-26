use crate::{Benchmark, benchmarks::merge};
#[allow(dead_code)]
mod email;
mod json;
mod url_pattern;

pub fn benchmarks() -> Vec<Benchmark> {
    merge([
        // ("email", email::benchmarks()),
        ("url_pattern", url_pattern::benchmarks()),
        ("json", json::benchmarks()),
    ])
}

use crate::{benchmarks::merge, Benchmark};

mod keccak;
mod sha2;

pub fn benchmarks() -> Vec<Benchmark> {
    merge([("sha2", sha2::benchmarks()), ("keccak", keccak::benchmarks())])
}

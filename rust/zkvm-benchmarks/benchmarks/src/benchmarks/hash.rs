use lazy_static::lazy_static;

use crate::{benchmarks::merge, Benchmark};

mod keccak;
mod sha2;

lazy_static! {
    pub static ref BENCHMARKS: Vec<Benchmark> =
        merge([("sha2", sha2::BENCHMARKS.clone()), ("keccak", keccak::BENCHMARKS.clone())]);
}

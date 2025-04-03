use crate::{benchmarks::merge, Benchmark};

mod hash;

mod rsa;

pub fn benchmarks() -> Vec<Benchmark> {
    merge([
        ("sha2", hash::sha2::benchmarks()),
        ("keccak", hash::keccak::benchmarks()),
        ("rsa", rsa::benchmarks()),
    ])
}

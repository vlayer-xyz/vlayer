use alloy_primitives::keccak256;

use crate::Benchmark;

fn empty() {
    keccak256([]);
}

fn one_block() {
    keccak256([0; 32]);
}

fn one_kb() {
    keccak256([0; 1_024]);
}

fn eight_kb() {
    keccak256([0; 8_192]);
}

pub fn benchmarks() -> Vec<Benchmark> {
    vec![
        Benchmark::new("empty", empty, 1_915),
        Benchmark::new("one_block", one_block, 2_121),
        Benchmark::new("one_kb", one_kb, 18_449),
        Benchmark::new("eight_kb", eight_kb, 138_789),
    ]
}

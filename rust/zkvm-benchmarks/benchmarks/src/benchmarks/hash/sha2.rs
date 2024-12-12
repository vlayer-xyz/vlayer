use sha2::{Digest, Sha256};

use crate::Benchmark;

fn empty() {
    Sha256::digest([]);
}

fn one_block() {
    Sha256::digest([0; 32]);
}

fn one_kb() {
    Sha256::digest([0; 1_024]);
}

fn eight_kb() {
    Sha256::digest([0; 8_192]);
}

pub fn benchmarks() -> Vec<Benchmark> {
    vec![
        Benchmark::new("empty", empty, 782),
        Benchmark::new("one_block", one_block, 714),
        Benchmark::new("one_kb", one_kb, 2_739),
        Benchmark::new("eight_kb", eight_kb, 12_745),
    ]
}

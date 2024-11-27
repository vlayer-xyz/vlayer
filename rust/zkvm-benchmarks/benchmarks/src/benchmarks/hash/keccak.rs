use alloy_primitives::keccak256;

use crate::{Benchmark, WorkloadResult};

fn empty() -> WorkloadResult {
    keccak256([]);

    Ok(())
}

fn one_block() -> WorkloadResult {
    keccak256([0; 32]);

    Ok(())
}

fn one_kb() -> WorkloadResult {
    keccak256([0; 1_024]);

    Ok(())
}

fn eight_kb() -> WorkloadResult {
    keccak256([0; 8_192]);

    Ok(())
}

pub fn benchmarks() -> Vec<Benchmark> {
    vec![
        Benchmark::new("empty", empty, 26_005),
        Benchmark::new("one_block", one_block, 26_211),
        Benchmark::new("one_kb", one_kb, 211_176),
        Benchmark::new("eight_kb", eight_kb, 1_608_339),
    ]
}

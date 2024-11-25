use alloy_primitives::keccak256;
use lazy_static::lazy_static;

use crate::{Benchmark, Workload, WorkloadResult};

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

lazy_static! {
    pub static ref BENCHMARKS: Vec<Benchmark> = vec![
        Benchmark::new("empty", empty as Workload, 26_005),
        Benchmark::new("one_block", one_block as Workload, 26_211),
        Benchmark::new("one_kb", one_kb as Workload, 211_176),
        Benchmark::new("eight_kb", eight_kb as Workload, 1_608_339)
    ];
}

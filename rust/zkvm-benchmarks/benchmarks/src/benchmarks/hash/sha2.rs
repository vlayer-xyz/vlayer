use lazy_static::lazy_static;
use sha2::{Digest, Sha256};

use crate::{Benchmark, Workload, WorkloadResult};

fn empty() -> WorkloadResult {
    Sha256::digest([]);

    Ok(())
}

fn one_block() -> WorkloadResult {
    Sha256::digest([0; 32]);

    Ok(())
}

fn one_kb() -> WorkloadResult {
    Sha256::digest([0; 1_024]);

    Ok(())
}

fn eight_kb() -> WorkloadResult {
    Sha256::digest([0; 8_192]);

    Ok(())
}

lazy_static! {
    pub static ref BENCHMARKS: Vec<Benchmark> = vec![
        Benchmark::new("empty", empty as Workload, 547),
        Benchmark::new("one_block", one_block as Workload, 650),
        Benchmark::new("one_kb", one_kb as Workload, 2_641),
        Benchmark::new("eight_kb", eight_kb as Workload, 12_745)
    ];
}

use sha2::{Digest, Sha256};

use crate::{Benchmark, WorkloadResult};

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

pub fn benchmarks() -> Vec<Benchmark> {
    vec![
        Benchmark::new("empty", empty, 547),
        Benchmark::new("one_block", one_block, 778),
        Benchmark::new("one_kb", one_kb, 2_641),
        Benchmark::new("eight_kb", eight_kb, 12_745),
    ]
}

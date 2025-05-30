#![allow(
    clippy::needless_pass_by_value,
    clippy::expect_used,
    clippy::unwrap_used
)]

use std::fmt::Display;

use benchmarks::benchmarks;
use derive_more::Debug;
use derive_new::new;
use risc0_zkvm::guest::env::cycle_count;
use serde::{Deserialize, Serialize};

mod benchmarks;
#[allow(dead_code)] // used in `build.rs`
pub mod build_utils;

pub struct Runner(Vec<Benchmark>);

impl Runner {
    pub fn new() -> Self {
        Self(benchmarks())
    }

    pub fn run(self) -> Vec<BenchmarkResult> {
        let mut results = Vec::with_capacity(self.0.len());
        for benchmark in self.0 {
            let result = benchmark.run();
            results.push(result);
        }
        results
    }
}

impl Default for Runner {
    fn default() -> Self {
        Self::new()
    }
}

#[derive(Debug)]
struct Benchmark {
    name: String,
    #[debug(skip)]
    workload: Box<dyn FnOnce()>,
    snapshot_cycles: u64,
}

impl Benchmark {
    fn new(
        name: impl Into<String>,
        workload: impl FnOnce() + 'static,
        snapshot_cycles: u64,
    ) -> Self {
        Self {
            name: name.into(),
            workload: Box::new(workload),
            snapshot_cycles,
        }
    }

    pub fn nest(self, module: &str) -> Benchmark {
        Benchmark {
            name: format!("{}::{}", module, self.name),
            ..self
        }
    }

    fn run(self) -> BenchmarkResult {
        let start = cycle_count();
        (self.workload)();
        let end = cycle_count();
        let cycles = end - start;

        BenchmarkResult::new(self.name, cycles, self.snapshot_cycles)
    }
}

#[derive(Debug, Clone, new, Serialize, Deserialize, Eq, PartialEq)]
pub struct BenchmarkResult {
    pub name: String,
    pub actual_cycles: u64,
    pub snapshot_cycles: u64,
}

impl Display for BenchmarkResult {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}: {} cycles (snapshot: {} cycles)",
            self.name, self.actual_cycles, self.snapshot_cycles
        )
    }
}

#![allow(clippy::needless_pass_by_value)]

use std::fmt::Display;

use benchmarks::benchmarks;
use derive_more::Debug;
use derive_new::new;
use serde::{Deserialize, Serialize};

mod benchmarks;

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
        #[cfg(feature = "risc0")]
        let start = risc0_zkvm::guest::env::cycle_count();
        #[cfg(feature = "sp1")]
        println!("cycle-tracker-report-start: {}", self.name);

        (self.workload)();
        #[cfg(feature = "risc0")]
        let end = risc0_zkvm::guest::env::cycle_count();
        #[cfg(feature = "sp1")]
        println!("cycle-tracker-report-end: {}", self.name);

        #[cfg(feature = "risc0")]
        let cycles = end - start;

        #[cfg(not(feature = "risc0"))]
        let cycles = 0;

        BenchmarkResult::new(self.name, cycles, self.snapshot_cycles)
    }
}

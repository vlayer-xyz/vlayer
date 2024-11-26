use std::fmt::Display;

use benchmarks::BENCHMARKS;
use derive_new::new;
use risc0_zkvm::guest::env;
use serde::{Deserialize, Serialize};

mod benchmarks;

pub struct Runner(Vec<Benchmark>);

type WorkloadResult = Result<(), ()>;
type Workload = fn() -> WorkloadResult;

impl Runner {
    pub fn new() -> Self {
        Self(BENCHMARKS.clone())
    }

    pub fn run(self) -> Vec<BenchmarkResult> {
        let mut results = Vec::with_capacity(self.0.len());
        for benchmark in self.0 {
            let result = benchmark.run().expect("benchmark failed");
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

#[derive(Debug, Clone)]
pub struct Benchmark {
    name: String,
    workload: Workload,
    snapshot_cycles: u64,
}

impl Benchmark {
    pub fn new(name: impl Into<String>, workload: Workload, snapshot_cycles: u64) -> Self {
        Self {
            name: name.into(),
            workload,
            snapshot_cycles,
        }
    }

    pub fn nest(self, module: &str) -> Benchmark {
        Benchmark {
            name: format!("{}::{}", module, self.name),
            ..self
        }
    }

    fn run(self) -> Result<BenchmarkResult, ()> {
        let start = env::cycle_count();
        (self.workload)()?;
        let end = env::cycle_count();
        let cycles = end - start;

        Ok(BenchmarkResult::new(self.name, cycles, self.snapshot_cycles))
    }
}

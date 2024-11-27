use std::fmt::Display;

use benchmarks::benchmarks;
use derive_more::Debug;
use derive_new::new;
use risc0_zkvm::guest::env;
use serde::{Deserialize, Serialize};

mod benchmarks;

type WorkloadResult = Result<(), ()>;

trait Workload {
    fn setup(&mut self) {}
    fn run(&mut self) -> WorkloadResult;
}

impl<F> Workload for F
where
    F: FnMut() -> WorkloadResult,
{
    fn run(&mut self) -> WorkloadResult {
        (self)()
    }
}

pub struct Runner(Vec<Benchmark>);

impl Runner {
    pub fn new() -> Self {
        Self(benchmarks())
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

#[derive(Debug)]
struct Benchmark {
    name: String,
    #[debug(skip)]
    workload: Box<dyn Workload>,
    snapshot_cycles: u64,
}

impl Benchmark {
    fn new(
        name: impl Into<String>,
        workload: impl Workload + 'static,
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

    fn run(mut self) -> Result<BenchmarkResult, ()> {
        self.workload.setup();
        let start = env::cycle_count();
        self.workload.run()?;
        let end = env::cycle_count();
        let cycles = end - start;

        Ok(BenchmarkResult::new(self.name, cycles, self.snapshot_cycles))
    }
}

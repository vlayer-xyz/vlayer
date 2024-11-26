use std::fmt::Display;

use benchmarks::BENCHMARKS;
use risc0_zkvm::guest::env;
use thousands::Separable;
mod benchmarks;

// Cycle count is non-deterministic so we ignore differences up to 5% when comparing
const TOLERANCE: f64 = 1.05;

pub struct BenchmarkRunner(Vec<Benchmark>);

type WorkloadResult = Result<(), ()>;
type Workload = fn() -> WorkloadResult;

struct BenchmarkResult {
    name: String,
    actual_cycles: u64,
    snapshot_cycles: u64,
}

impl Display for BenchmarkResult {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}: actual: {} snapshot: {} difference: {}",
            self.name,
            self.actual_cycles.separate_with_underscores(),
            self.snapshot_cycles.separate_with_underscores(),
            (self.actual_cycles as i64 - self.snapshot_cycles as i64).separate_with_underscores()
        )
    }
}

impl BenchmarkRunner {
    pub fn new() -> Self {
        Self(BENCHMARKS.clone())
    }

    pub fn run_all(self) -> Result<(), u64> {
        let results: Vec<Result<BenchmarkResult, ()>> =
            self.0.into_iter().map(Benchmark::run).collect();
        println!("Run total of: {}", results.len());

        let (passed, failed): (Vec<_>, Vec<_>) = results.into_iter().partition(|r| r.is_ok());

        println!("Passed total of: {}", passed.len());

        if !failed.is_empty() {
            println!("Failed total of: {}", failed.len());
            return Err(failed.len() as u64);
        }
        println!("Results:");
        passed
            .into_iter()
            .map(Result::unwrap)
            .for_each(|r| println!("{}", r));

        Ok(())
    }
}

impl Default for BenchmarkRunner {
    fn default() -> Self {
        Self::new()
    }
}

fn apply_tolerance(cycles: u64) -> u64 {
    (cycles as f64 * TOLERANCE) as u64
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

        let actual_cycles = end - start;
        if actual_cycles > apply_tolerance(self.snapshot_cycles) {
            eprintln!(
                "Benchmark {} failed with result: {} > {}",
                self.name, actual_cycles, self.snapshot_cycles
            );
            return Err(());
        }

        Ok(BenchmarkResult {
            name: self.name,
            actual_cycles,
            snapshot_cycles: self.snapshot_cycles,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn thousands_separated() {
        let result = BenchmarkResult {
            name: "test".to_string(),
            actual_cycles: 1_010,
            snapshot_cycles: 1_000,
        };
        assert_eq!(result.to_string(), "test: actual: 1_010 snapshot: 1_000 difference: 10");
    }

    #[test]
    fn tolerance() {
        assert_eq!(apply_tolerance(1), 1);
        assert_eq!(apply_tolerance(100), 105);
        assert_eq!(apply_tolerance(1_027), 1_078);
    }
}

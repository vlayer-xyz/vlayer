use std::fmt::Display;

use benchmarks::BENCHMARKS;
use risc0_zkvm::guest::env;
use tabled::{Table, Tabled};
use thousands::Separable;
mod benchmarks;

// Cycle count is non-deterministic so we ignore differences up to 10% when comparing.
// 5% was tried and was not enough
const TOLERANCE: f64 = 1.1;

pub struct Runner(Vec<Benchmark>);

type WorkloadResult = Result<(), ()>;
type Workload = fn() -> WorkloadResult;

struct CycleCount(u64);

impl Display for CycleCount {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.0.separate_with_underscores().fmt(f)
    }
}

struct CycleDiff(i64);

impl Display for CycleDiff {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.0.separate_with_underscores().fmt(f)
    }
}

#[derive(Tabled)]
struct BenchmarkResult {
    name: String,
    actual_cycles: CycleCount,
    snapshot_cycles: CycleCount,
    diff: CycleDiff,
}

impl BenchmarkResult {
    fn new(name: String, actual_cycles: u64, snapshot_cycles: u64) -> Self {
        Self {
            name,
            actual_cycles: CycleCount(actual_cycles),
            snapshot_cycles: CycleCount(snapshot_cycles),
            diff: CycleDiff(actual_cycles as i64 - snapshot_cycles as i64),
        }
    }
}

impl Runner {
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

        let results: Vec<_> = passed.into_iter().map(|r| r.unwrap()).collect();
        println!("{}", Table::new(results));

        Ok(())
    }
}

impl Default for Runner {
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

        Ok(BenchmarkResult::new(self.name, actual_cycles, self.snapshot_cycles))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn prints_results_table() {
        let results = vec![BenchmarkResult::new("test".to_string(), 1_010, 1_000)];

        let table = format!("{}", Table::new(results));

        assert_eq!(
            table,
            "+------+---------------+-----------------+------+
| name | actual_cycles | snapshot_cycles | diff |
+------+---------------+-----------------+------+
| test | 1_010         | 1_000           | 10   |
+------+---------------+-----------------+------+"
        );
    }

    #[test]
    fn tolerance() {
        assert_eq!(apply_tolerance(1), 1);
        assert_eq!(apply_tolerance(100), 110);
        assert_eq!(apply_tolerance(1_027), 1_129);
    }
}

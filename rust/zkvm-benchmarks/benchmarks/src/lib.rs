use std::fmt::Display;

use benchmarks::BENCHMARKS;
use risc0_zkvm::guest::env;
use thousands::Separable;
mod benchmarks;

pub struct BenchmarkRunner(Vec<Benchmark>);

type WorkloadResult = Result<(), ()>;
type Workload = fn() -> WorkloadResult;

struct BenchmarkResult {
    name: String,
    used_cycles: u64,
    limit_cycles: u64,
}

impl Display for BenchmarkResult {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}: {} / {}",
            self.name,
            self.used_cycles.separate_with_underscores(),
            self.limit_cycles.separate_with_underscores()
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

#[derive(Debug, Clone)]
pub struct Benchmark {
    name: String,
    workload: Workload,
    total_cycles_limit: u64,
}

impl Benchmark {
    pub fn new(name: impl Into<String>, workload: Workload, total_cycles_limit: u64) -> Self {
        Self {
            name: name.into(),
            workload,
            total_cycles_limit,
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

        let total_cycles = end - start;
        if total_cycles > self.total_cycles_limit {
            eprintln!(
                "Benchmark {} failed with result: {} > {}",
                self.name, total_cycles, self.total_cycles_limit
            );
            return Err(());
        }

        Ok(BenchmarkResult {
            name: self.name,
            used_cycles: total_cycles,
            limit_cycles: self.total_cycles_limit,
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
            used_cycles: 1_000,
            limit_cycles: 1_000_000,
        };
        assert_eq!(result.to_string(), "test: 1_000 / 1_000_000");
    }
}

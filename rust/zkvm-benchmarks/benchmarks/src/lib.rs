use std::fmt::Display;

use benchmarks::hello;
use risc0_zkvm::guest::env;

mod benchmarks;

pub struct BenchmarkRunner(Vec<Benchmark>);

type WorkloadResult = Result<(), ()>;
type Workload = fn() -> WorkloadResult;

struct BenchmarkResult {
    name: &'static str,
    used_cycles: u64,
    limit_cycles: u64,
}

impl Display for BenchmarkResult {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}: {}/{}", self.name, self.used_cycles, self.limit_cycles)
    }
}

impl BenchmarkRunner {
    pub fn new() -> Self {
        let benchmarks = [Benchmark::new("hello", hello as Workload, 10)];

        Self(benchmarks.into())
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

pub struct Benchmark {
    name: &'static str,
    workload: Workload,
    total_cycles_limit: u64,
}

impl Benchmark {
    fn new(name: &'static str, workload: Workload, total_cycles_limit: u64) -> Self {
        Self {
            name,
            workload,
            total_cycles_limit,
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

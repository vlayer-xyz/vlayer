#![no_main]

risc0_zkvm::guest::entry!(main);
use risc0_zkvm::guest::env;
use zkvm_benchmarks::BenchmarkRunner;

fn main() {
    let runner: BenchmarkRunner = Default::default();
    let mut number_of_failed_benchmarks = 0;

    if let Err(errors) = runner.run_all() {
        number_of_failed_benchmarks = errors;
    }

    env::commit(&number_of_failed_benchmarks);
}

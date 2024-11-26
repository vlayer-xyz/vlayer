#![no_main]

risc0_zkvm::guest::entry!(main);
use risc0_zkvm::guest::env;
use zkvm_benchmarks::Runner;

fn main() {
    let runner: Runner = Default::default();

    env::commit(&runner.run());
}

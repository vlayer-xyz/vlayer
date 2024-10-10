#![no_main]

risc0_zkvm::guest::entry!(main);

use risc0_zkvm::guest::env;

fn main() {
    let start_cycles = env::cycle_count();

    let end_cycles = env::cycle_count();

    eprintln!("vlayer-benchmark::total={}", end_cycles - start_cycles);
}

#![no_main]

risc0_zkvm::guest::entry!(main);

use risc0_zkvm::guest::env;
use zkvm_bencher_types::GuestOutput;

fn main() {
    let start_cycles = env::cycle_count();

    // Code goes here

    let end_cycles = env::cycle_count();

    let output = GuestOutput {
        total_cycles: end_cycles - start_cycles,
    };

    println!("vlayer-benchmark::guest_output={:?}", &output);
    let output: Vec<u8> = output.into();
    env::commit_slice(output.as_slice());
}

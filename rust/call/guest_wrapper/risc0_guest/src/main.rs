#![no_main]

risc0_zkvm::guest::entry!(main);

use alloy_sol_types::SolValue;
use call_guest::{Guest, Input};
use risc0_zkvm::guest::env;

fn main() {
    let Input {
        multi_evm_input,
        call,
        start_execution_location,
        augmentors,
    } = env::read();

    let output = Guest::new(multi_evm_input, start_execution_location).run(call, None);

    env::commit_slice(&output.execution_commitment.abi_encode());
    env::commit_slice(&output.evm_call_result);
}

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
        chain_proofs,
    } = env::read();

    let guest = Guest::new(multi_evm_input, start_execution_location, &chain_proofs);
    let runtime = tokio::runtime::Builder::new_current_thread()
        .build()
        .expect("failed to create tokio runtime");
    let output = runtime.block_on(guest.run(&call));

    env::commit_slice(&output.call_assumptions.abi_encode());
    env::commit_slice(&output.evm_call_result);
}

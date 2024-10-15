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
        chain_id_to_chain_proof
    } = env::read();

    let output = Guest::new(multi_evm_input, start_execution_location, chain_id_to_chain_proof).run(&call);

    env::commit_slice(&output.call_assumptions.abi_encode());
    env::commit_slice(&output.evm_call_result);
}

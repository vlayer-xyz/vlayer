#![no_main]

risc0_zkvm::guest::entry!(main);

use alloy_sol_types::SolValue;
use guest::Guest;
use risc0_zkvm::guest::env;
use vlayer_engine::io::Input;

pub mod db;
pub mod guest;

fn main() {
    let Input {
        multi_evm_input,
        call,
        start_execution_location,
    } = env::read();
    let start_evm_input = multi_evm_input
        .get(&start_execution_location)
        .expect("cannot get start evm input")
        .to_owned(); // TODO: Remove clone and convert this object into MultiEnv
    let output = Guest::new(start_evm_input, start_execution_location).run(call);

    env::commit_slice(&output.execution_commitment.abi_encode());
    env::commit_slice(&output.evm_call_result);
}

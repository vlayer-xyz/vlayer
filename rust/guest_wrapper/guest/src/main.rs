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
        evm_input, call, ..
    } = env::read();

    let output = Guest::new(evm_input).run(call);

    env::commit_slice(&output.execution_commitment.abi_encode());
    env::commit_slice(&output.evm_call_result);
}

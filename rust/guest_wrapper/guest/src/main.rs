#![no_main]

risc0_zkvm::guest::entry!(main);

use alloy_sol_types::SolValue;
use guest::Guest;
use risc0_zkvm::guest::env;
use vlayer_engine::io::Input;

pub mod guest;

fn main() {
    let Input { evm_input, call } = env::read();

    let output = Guest::new(evm_input).run(call);

    let mut result: Vec<u8> = Vec::new();
    result.extend(&output.execution_commitment.abi_encode());
    result.extend(&output.evm_call_result);

    env::commit(&result);
}

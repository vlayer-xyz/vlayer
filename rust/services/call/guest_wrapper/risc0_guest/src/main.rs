#![no_main]

risc0_zkvm::guest::entry!(main);

use alloy_sol_types::SolValue;
use chain_guest_wrapper::GUEST_ELF;
use risc0_zkvm::guest::env;

#[tokio::main(flavor = "current_thread")]
async fn main() {
    let input = env::read();

    let output = call_guest::main(input, GUEST_ELF.id).await;

    env::commit_slice(&output.call_assumptions.abi_encode());
    env::commit_slice(&output.evm_call_result);
}

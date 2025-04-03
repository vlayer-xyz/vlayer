#![no_main]
sp1_zkvm::entrypoint!(main);

use alloy_sol_types::SolValue;

#[tokio::main(flavor = "current_thread")]
async fn main() {
    let input = sp1_zkvm::io::read();

    let output = call_guest::main(input, vec![]).await;

    sp1_zkvm::io::commit_slice(&output.call_assumptions.abi_encode());
    sp1_zkvm::io::commit_slice(&output.evm_call_result);
}

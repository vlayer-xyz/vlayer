#![no_main]

risc0_zkvm::guest::entry!(main);

use alloy_sol_types::SolValue;
use risc0_zkvm::guest::env;

include!(concat!(env!("OUT_DIR"), "/guest_id.rs"));

#[tokio::main(flavor = "current_thread")]
async fn main() {
    let input = env::read();

    let chain_guest_ids = CHAIN_GUEST_IDS.into_iter().map(Into::into);
    let output = call_guest::main(input, chain_guest_ids).await;

    env::commit_slice(&output.call_assumptions.abi_encode());
    env::commit_slice(&output.evm_call_result);
}

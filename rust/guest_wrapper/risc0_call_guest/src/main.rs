#![no_main]

risc0_zkvm::guest::entry!(main);

use alloy_sol_types::SolValue;
use risc0_zkvm::{guest::env, sha::Digest};

include!(concat!(env!("OUT_DIR"), "/guest_id.rs"));

#[tokio::main(flavor = "current_thread")]
async fn main() {
    let input = env::read();

    let output = call_guest::main(input, Digest::from_bytes(CHAIN_GUEST_ELF_ID)).await;

    env::commit_slice(&output.call_assumptions.abi_encode());
    env::commit_slice(&output.evm_call_result);
}

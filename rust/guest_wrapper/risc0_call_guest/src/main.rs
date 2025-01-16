#![no_main]

risc0_zkvm::guest::entry!(main);

use alloy_sol_types::SolValue;
use risc0_zkvm::{guest::env, sha::Digest};

#[tokio::main(flavor = "current_thread")]
async fn main() {
    let input = env::read();
    let chain_guest_elf_id: Digest = (*include_bytes!("../../artifacts/chain_guest/elf_id")).into();

    let output = call_guest::main(input, chain_guest_elf_id).await;

    env::commit_slice(&output.call_assumptions.abi_encode());
    env::commit_slice(&output.evm_call_result);
}

#![no_main]

risc0_zkvm::guest::entry!(main);

use alloy_sol_types::SolValue;
use call_guest::{GuestBuilder, ChainProofVerifier, Input};
use chain_client::CachedClient;
use chain_common::Risc0Verifier;
use risc0_zkvm::guest::env;

pub const CHAIN_GUEST_ID: [u32; 8] = [3128717491, 1801560807, 26647913, 65464827, 2627242429, 1252976105, 837527689, 1604114230];

#[tokio::main(flavor = "current_thread")]
async fn main() {
    let Input {
        multi_evm_input,
        call,
        start_execution_location,
        chain_proofs,
    } = env::read();

    let chain_proof_client = CachedClient::new(chain_proofs);
    let chain_proof_verifier = ChainProofVerifier::new(Risc0Verifier, CHAIN_GUEST_ID.into());
    let guest_builder = GuestBuilder::new(chain_proof_client, chain_proof_verifier);
    let guest = guest_builder.build_guest(multi_evm_input, start_execution_location).await;
    let output = guest.run(&call).await;

    env::commit_slice(&output.call_assumptions.abi_encode());
    env::commit_slice(&output.evm_call_result);
}

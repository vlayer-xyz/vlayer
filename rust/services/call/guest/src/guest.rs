use call_engine::{
    verifier::{chain_proof, time_travel, travel_call, zk_proof},
    GuestOutput, Input,
};
use chain_client::CachedClient;
use env::{verify_input, VerifiedEnv};
use risc0_zkvm::sha::Digest;

mod env;
#[cfg(test)]
mod tests;

pub async fn main(
    Input {
        multi_evm_input,
        start_execution_location,
        chain_proofs,
        call,
    }: Input,
    chain_guest_ids: impl IntoIterator<Item = Digest>,
) -> GuestOutput {
    let chain_client = CachedClient::new(chain_proofs);
    let chain_proof_verifier = chain_proof::Verifier::new(chain_guest_ids, zk_proof::GuestVerifier);
    let time_travel_verifier = time_travel::Verifier::new(chain_client, chain_proof_verifier);
    let travel_call_verifier = travel_call::Verifier::new(time_travel_verifier);
    let verified_input = verify_input(travel_call_verifier, multi_evm_input).await;
    VerifiedEnv::new(verified_input, start_execution_location).exec_call(&call)
}

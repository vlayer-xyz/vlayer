use call_engine::{
    verifier::{chain_proof, guest_input, zk_proof},
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
    chain_guest_id: Digest,
) -> GuestOutput {
    let input_verifier = chain_proofs.map(|chain_proofs| {
        let chain_client = CachedClient::new(chain_proofs);
        let chain_proof_verifier =
            chain_proof::ZkVerifier::new(chain_guest_id, zk_proof::GuestVerifier);
        guest_input::ZkVerifier::new(chain_client, chain_proof_verifier)
    });
    let verified_input = verify_input(input_verifier, multi_evm_input).await;
    VerifiedEnv::new(verified_input, start_execution_location).exec_call(&call)
}

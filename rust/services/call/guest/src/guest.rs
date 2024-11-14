use call_engine::{
    verifier::{chain_proof, guest_input, zk_proof},
    GuestOutput, Input,
};
use chain_client::CachedClient;
use env::{assert_input_coherency, verify_input, VerifiedEnv};
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
    let verified_input = if let Some(chain_proofs) = chain_proofs {
        let chain_client = CachedClient::new(chain_proofs);
        let chain_proof_verifier =
            chain_proof::ZkVerifier::new(chain_guest_id, zk_proof::GuestVerifier);
        let input_verifier = guest_input::ZkVerifier::new(&chain_client, &chain_proof_verifier);
        verify_input(input_verifier, multi_evm_input).await
    } else {
        assert_input_coherency(multi_evm_input)
    };
    VerifiedEnv::new(verified_input, start_execution_location).exec_call(&call)
}

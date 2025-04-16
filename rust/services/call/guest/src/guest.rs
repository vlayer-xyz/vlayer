use call_engine::{
    CallAssumptions, GuestOutput, Input,
    evm::env::cached::CachedEvmEnv,
    travel_call::Executor as TravelCallExecutor,
    verifier::{
        teleport, time_travel,
        travel_call::{self, IVerifier},
    },
};
use chain_client::{CachedClient, ChainProofCache};
use common::verifier::zk_proof;
use env::create_envs_from_input;
use optimism::client::factory::cached::OpOutputCache;
use risc0_zkvm::sha::Digest;

use crate::db::GuestDb;

mod env;
#[cfg(test)]
mod tests;

type GuestTravelCallVerifier = travel_call::Verifier<
    GuestDb,
    time_travel::Verifier<CachedClient, chain_common::verifier::Verifier<zk_proof::GuestVerifier>>,
    teleport::Verifier,
>;

#[allow(clippy::expect_used)]
pub async fn main(
    Input {
        multi_evm_input,
        start_execution_location,
        chain_proofs,
        call,
        op_output_cache,
        is_vlayer_test,
    }: Input,
    chain_guest_ids: impl IntoIterator<Item = Digest>,
) -> GuestOutput {
    multi_evm_input.assert_coherency();

    let envs = create_envs_from_input(multi_evm_input);
    let cached_envs = CachedEvmEnv::from_envs(envs);

    let travel_call_verifier =
        build_guest_travel_call_verifier(chain_proofs, chain_guest_ids, op_output_cache);
    travel_call_verifier
        .verify(&cached_envs, start_execution_location)
        .await
        .expect("travel call verification failed");

    let evm_call_result =
        TravelCallExecutor::new(&cached_envs, start_execution_location, is_vlayer_test)
            .call(&call)
            .expect("travel call execution failed")
            .output;

    let start_env = cached_envs
        .get(start_execution_location)
        .expect("cannot get start evm env");

    let call_assumptions = CallAssumptions::new(
        start_execution_location.chain_id,
        start_env.header(),
        call.to,
        call.selector(),
    );

    GuestOutput::new(call_assumptions, evm_call_result)
}

fn build_guest_travel_call_verifier(
    chain_proofs: ChainProofCache,
    chain_guest_ids: impl IntoIterator<Item = Digest>,
    op_output_cache: OpOutputCache,
) -> GuestTravelCallVerifier {
    let chain_client = CachedClient::new(chain_proofs);
    let chain_proof_verifier =
        chain_common::verifier::Verifier::new(chain_guest_ids, zk_proof::GuestVerifier);
    let time_travel_verifier = time_travel::Verifier::new(Some(chain_client), chain_proof_verifier);
    let op_client_factory = optimism::client::factory::cached::Factory::new(op_output_cache);
    let teleport_verifier = teleport::Verifier::new(op_client_factory);
    travel_call::Verifier::new(time_travel_verifier, teleport_verifier)
}

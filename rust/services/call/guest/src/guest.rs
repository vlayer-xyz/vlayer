use std::sync::Arc;

use call_engine::{
    evm::{
        env::{cached::CachedEvmEnv, location::ExecutionLocation, EvmEnv},
        input::MultiEvmInput,
    },
    io::{Call, GuestOutput, Input},
    travel_call_executor::TravelCallExecutor,
    verifier::{GuestInputVerifier, GuestVerifier, ZkChainProofVerifier, ZkGuestInputVerifier},
    CallAssumptions,
};
use chain_client::CachedClient;
use chain_guest_wrapper::RISC0_CHAIN_GUEST_ID;
use derive_new::new;

use crate::db::wrap_state::WrapStateDb;

#[cfg(test)]
mod tests;

#[derive(Debug, Clone, Default, new)]
struct GuestEnv {
    multi_evm_input: MultiEvmInput,
    start_exec_location: ExecutionLocation,
}

impl GuestEnv {
    #[must_use]
    pub async fn verify_env(self, verifier: impl GuestInputVerifier) -> VerifiedGuestEnv {
        let GuestEnv {
            multi_evm_input,
            start_exec_location,
        } = self;

        multi_evm_input.assert_coherency();
        verifier
            .verify(&multi_evm_input)
            .await
            .expect("invalid guest input");

        let multi_evm_env = CachedEvmEnv::from_envs(multi_evm_input.into());
        VerifiedGuestEnv::new(multi_evm_env, start_exec_location)
    }
}

#[derive(new)]
struct VerifiedGuestEnv {
    multi_evm_env: CachedEvmEnv<WrapStateDb>,
    start_exec_location: ExecutionLocation,
}

impl VerifiedGuestEnv {
    fn get_start_env(&self) -> Arc<EvmEnv<WrapStateDb>> {
        self.multi_evm_env
            .get(self.start_exec_location)
            .expect("cannot get start evm env")
    }

    #[must_use]
    pub fn exec_call(self, call: &Call) -> GuestOutput {
        let evm_call_result = TravelCallExecutor::new(&self.multi_evm_env)
            .call(call, self.start_exec_location)
            .expect("travel call executin failed")
            .output;
        let start_env = self.get_start_env();
        let call_assumptions = CallAssumptions::new(start_env.header(), call.to, call.selector());
        GuestOutput::new(call_assumptions, evm_call_result)
    }
}

pub async fn main(
    Input {
        multi_evm_input,
        start_execution_location,
        chain_proofs,
        call,
    }: Input,
) -> GuestOutput {
    let chain_client = CachedClient::new(chain_proofs);
    let chain_proof_verifier = ZkChainProofVerifier::new(RISC0_CHAIN_GUEST_ID, GuestVerifier);
    let input_verifier = ZkGuestInputVerifier::new(chain_client, chain_proof_verifier);
    GuestEnv::new(multi_evm_input, start_execution_location)
        .verify_env(input_verifier)
        .await
        .exec_call(&call)
}

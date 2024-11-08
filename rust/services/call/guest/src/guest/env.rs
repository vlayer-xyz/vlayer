use std::sync::Arc;

use call_engine::{
    evm::{
        env::{cached::CachedEvmEnv, location::ExecutionLocation, EvmEnv},
        input::MultiEvmInput,
    },
    travel_call_executor::TravelCallExecutor,
    verifier::guest_input::Verifier,
    Call, CallAssumptions, GuestOutput,
};

use crate::db::wrap_state::WrapStateDb;

pub struct VerifiedInput(MultiEvmInput);

pub async fn verify_input(
    verifier: impl Verifier,
    multi_evm_input: MultiEvmInput,
) -> VerifiedInput {
    multi_evm_input.assert_coherency();
    verifier
        .verify(&multi_evm_input)
        .await
        .expect("invalid guest input");

    VerifiedInput(multi_evm_input)
}

pub struct VerifiedEnv {
    multi_evm_env: CachedEvmEnv<WrapStateDb>,
    start_exec_location: ExecutionLocation,
}

impl VerifiedEnv {
    #[must_use]
    pub fn new(verified_input: VerifiedInput, start_exec_location: ExecutionLocation) -> Self {
        Self {
            multi_evm_env: CachedEvmEnv::from_envs(verified_input.0.into()),
            start_exec_location,
        }
    }

    fn get_start_env(&self) -> Arc<EvmEnv<WrapStateDb>> {
        self.multi_evm_env
            .get(self.start_exec_location)
            .expect("cannot get start evm env")
    }

    #[must_use]
    pub fn exec_call(self, call: &Call) -> GuestOutput {
        let evm_call_result = TravelCallExecutor::new(&self.multi_evm_env)
            .call(call, self.start_exec_location)
            .expect("travel call execution failed")
            .output;
        let start_env = self.get_start_env();
        let call_assumptions = CallAssumptions::new(start_env.header(), call.to, call.selector());
        GuestOutput::new(call_assumptions, evm_call_result)
    }
}

use std::sync::{Arc, RwLock};

use call_engine::{
    evm::{
        env::{
            cached::{CachedEvmEnv, MultiEvmEnv},
            location::ExecutionLocation,
            EvmEnv,
        },
        input::{EvmInput, MultiEvmInput},
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
    verifier
        .verify(&multi_evm_input)
        .await
        .expect("invalid guest input");

    assert_input_coherency(multi_evm_input)
}

pub fn assert_input_coherency(multi_evm_input: MultiEvmInput) -> VerifiedInput {
    multi_evm_input.assert_coherency();
    VerifiedInput(multi_evm_input)
}

fn create_env_from_input(input: EvmInput) -> EvmEnv<WrapStateDb> {
    let header = input.header.clone();
    let db = WrapStateDb::from(input);
    EvmEnv::new(db, header)
}

fn create_envs_from_input(input: MultiEvmInput) -> MultiEvmEnv<WrapStateDb> {
    RwLock::new(
        input
            .into_iter()
            .map(|(location, input)| {
                let chain_spec = &location.chain_id.try_into().expect("cannot get chain spec");
                #[allow(clippy::arc_with_non_send_sync)]
                let input = Arc::new(
                    create_env_from_input(input)
                        .with_chain_spec(chain_spec)
                        .unwrap(),
                );
                (location, input)
            })
            .collect(),
    )
}

pub struct VerifiedEnv {
    multi_evm_env: CachedEvmEnv<WrapStateDb>,
    start_exec_location: ExecutionLocation,
}

impl VerifiedEnv {
    #[must_use]
    pub fn new(verified_input: VerifiedInput, start_exec_location: ExecutionLocation) -> Self {
        let multi_evm_env = create_envs_from_input(verified_input.0);
        Self {
            multi_evm_env: CachedEvmEnv::from_envs(multi_evm_env),
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

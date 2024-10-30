use std::collections::HashMap;

use alloy_primitives::ChainId;
use call_engine::{
    evm::{
        env::{cached::CachedEvmEnv, location::ExecutionLocation},
        input::MultiEvmInput,
    },
    io::{Call, GuestOutput},
    travel_call_executor::TravelCallExecutor,
    CallAssumptions,
};
use chain_common::ChainProof;

use crate::db::wrap_state::WrapStateDb;

const VERIFY_CHAIN_PROOFS: bool = false;

pub struct Guest {
    start_execution_location: ExecutionLocation,
    evm_envs: CachedEvmEnv<WrapStateDb>,
}

impl Guest {
    #[must_use]
    pub fn new(
        multi_evm_input: MultiEvmInput,
        start_execution_location: ExecutionLocation,
        chain_proofs: &HashMap<ChainId, ChainProof>,
    ) -> Self {
        multi_evm_input.assert_coherency(chain_proofs, VERIFY_CHAIN_PROOFS);
        let multi_evm_env = multi_evm_input.into();
        let evm_envs = CachedEvmEnv::from_envs(multi_evm_env);

        Guest {
            evm_envs,
            start_execution_location,
        }
    }

    #[allow(clippy::unused_async)]
    pub async fn run(self, call: &Call) -> GuestOutput {
        let evm_call_result = TravelCallExecutor::new(&self.evm_envs)
            .call(call, self.start_execution_location)
            .unwrap();
        let start_evm_env = self
            .evm_envs
            .get(self.start_execution_location)
            .expect("cannot get start evm env");
        let call_assumptions =
            CallAssumptions::new(start_evm_env.header(), call.to, call.selector());

        GuestOutput {
            evm_call_result: evm_call_result.output,
            call_assumptions,
        }
    }
}

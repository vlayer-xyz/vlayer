use call_engine::{
    engine::Engine,
    evm::{
        env::{
            cached::{CachedEvmEnv, MultiEvmEnv},
            location::ExecutionLocation,
        },
        input::MultiEvmInput,
    },
    io::{Call, GuestOutput},
    CallAssumptions,
};

use crate::{db::wrap_state::WrapStateDb, input_validation::ValidatedMultiEvmInput};

pub struct Guest {
    start_execution_location: ExecutionLocation,
    evm_envs: CachedEvmEnv<WrapStateDb>,
}

impl Guest {
    #[must_use]
    pub fn new(
        multi_evm_input: MultiEvmInput,
        start_execution_location: ExecutionLocation,
    ) -> Self {
        let validated_multi_evm_input: ValidatedMultiEvmInput = multi_evm_input.into();
        let multi_evm_env = MultiEvmEnv::from(validated_multi_evm_input);
        let evm_envs = CachedEvmEnv::from_envs(multi_evm_env);

        Guest {
            evm_envs,
            start_execution_location,
        }
    }

    pub fn run(self, call: &Call) -> GuestOutput {
        let evm_call_result = Engine::new(&self.evm_envs)
            .call(call, self.start_execution_location)
            .unwrap();
        let start_evm_env = self
            .evm_envs
            .get(self.start_execution_location)
            .expect("cannot get start evm env");
        let call_assumptions =
            CallAssumptions::new(start_evm_env.header(), call.to, call.selector());

        GuestOutput {
            evm_call_result,
            call_assumptions,
        }
    }
}

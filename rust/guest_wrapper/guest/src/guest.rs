use crate::{db::wrap_state::WrapStateDb, input::ValidatedMultiEvmInput};
use vlayer_engine::{
    block_header::eth::EthBlockHeader,
    engine::Engine,
    evm::{
        env::{CachedEvmEnv, ExecutionLocation, MultiEvmEnv},
        input::MultiEvmInput,
    },
    io::{Call, GuestOutput},
    ExecutionCommitment,
};

pub struct Guest {
    start_execution_location: ExecutionLocation,
    evm_envs: CachedEvmEnv<WrapStateDb, EthBlockHeader>,
}

impl Guest {
    pub fn new(
        multi_evm_input: MultiEvmInput<EthBlockHeader>,
        start_execution_location: ExecutionLocation,
    ) -> Self {
        let validated_multi_evm_input: ValidatedMultiEvmInput<_> = multi_evm_input.into();
        let multi_evm_env = MultiEvmEnv::from(validated_multi_evm_input);
        let evm_envs = CachedEvmEnv::from_envs(multi_evm_env);

        Guest {
            evm_envs,
            start_execution_location,
        }
    }

    pub fn run(&self, call: Call) -> GuestOutput {
        let evm_call_result = Engine::default()
            .call(&call, self.start_execution_location.clone(), &self.evm_envs)
            .unwrap();
        let start_evm_env = self
            .evm_envs
            .get(self.start_execution_location)
            .expect("cannot get start evm env");
        let execution_commitment =
            ExecutionCommitment::new(start_evm_env.header(), call.to, call.selector());

        GuestOutput {
            evm_call_result,
            execution_commitment,
        }
    }
}

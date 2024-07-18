use crate::{db::wrap_state::WrapStateDb, input::ValidatedMultiEvmInput};
use revm::db::WrapDatabaseRef;
use vlayer_engine::{
    block_header::eth::EthBlockHeader,
    engine::Engine,
    evm::{
        env::{ExecutionLocation, MultiEvmEnv},
        input::MultiEvmInput,
    },
    io::{Call, GuestOutput},
    ExecutionCommitment,
};

pub struct Guest {
    start_execution_location: ExecutionLocation,
    multi_evm_env: MultiEvmEnv<WrapDatabaseRef<WrapStateDb>, EthBlockHeader>,
}

impl Guest {
    pub fn new(
        multi_evm_input: MultiEvmInput<EthBlockHeader>,
        start_execution_location: ExecutionLocation,
    ) -> Self {
        let validated_multi_evm_input: ValidatedMultiEvmInput<_> = multi_evm_input.into();
        let multi_evm_env = MultiEvmEnv::from(validated_multi_evm_input);

        Guest {
            multi_evm_env,
            start_execution_location,
        }
    }

    pub fn run(&mut self, call: Call) -> GuestOutput {
        let start_evm_env = self
            .multi_evm_env
            .get_mut(&self.start_execution_location)
            .expect("cannot get evm env");

        let evm_call_result = Engine::default().call(&call, start_evm_env).unwrap();
        let execution_commitment =
            ExecutionCommitment::new(start_evm_env.header(), call.to, call.selector());

        GuestOutput {
            evm_call_result,
            execution_commitment,
        }
    }
}

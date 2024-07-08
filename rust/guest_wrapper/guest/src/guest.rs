use crate::{db::wrap_state::WrapStateDb, input::ValidatedEvmInput};
use vlayer_engine::{
    chain::spec::ChainSpec,
    engine::Engine,
    ethereum::EthBlockHeader,
    evm::{
        env::{EvmEnv, ExecutionLocation},
        input::MultiEvmInput,
    },
    io::{Call, GuestOutput},
    ExecutionCommitment,
};

pub struct Guest {
    env: EvmEnv<WrapStateDb, EthBlockHeader>,
}

impl Guest {
    pub fn new(
        multi_evm_input: MultiEvmInput<EthBlockHeader>,
        start_execution_location: ExecutionLocation,
    ) -> Self {
        let chain_spec = ChainSpec::try_from_config(start_execution_location.chain_id)
            .expect("cannot get chain spec");

        let start_evm_input = multi_evm_input
            .get(&start_execution_location)
            .expect("cannot get start evm input")
            .to_owned(); // TODO: Remove clone and convert this object into MultiEnv

        let validated_start_evm_input: ValidatedEvmInput<_> = start_evm_input.into();
        let env: EvmEnv<_, _> = validated_start_evm_input.into();

        let env = env
            .with_chain_spec(&chain_spec)
            .expect("cannot set chain spec");
        Guest { env }
    }

    pub fn run(&mut self, call: Call) -> GuestOutput {
        let evm_call_result = Engine::default().call(&call, &mut self.env).unwrap();
        let execution_commitment =
            ExecutionCommitment::new(self.env.header(), call.to, call.selector());

        GuestOutput {
            evm_call_result,
            execution_commitment,
        }
    }
}

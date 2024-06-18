use vlayer_steel::{
    config::ETH_SEPOLIA_CHAIN_SPEC,
    contract::engine::Engine,
    ethereum::EthBlockHeader,
    guest::{Call, Output},
    EvmEnv, EvmInput, StateDb,
};

pub struct Guest {
    env: EvmEnv<StateDb, EthBlockHeader>,
}

impl Guest {
    pub fn new(evm_input: EvmInput<EthBlockHeader>) -> Self {
        let env = evm_input
            .into_env()
            .with_chain_spec(&ETH_SEPOLIA_CHAIN_SPEC)
            .unwrap();

        Guest { env }
    }

    pub fn run(&self, call: Call) -> Output {
        Output {
            block_commitment: self.env.block_commitment(),
            evm_call_result: Engine::guest_evm_call(call_data, &self.env),
        }
    }
}

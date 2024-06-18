use vlayer_steel::{
    config::ETH_SEPOLIA_CHAIN_SPEC,
    contract::{call::guest_evm_call, CallTxData},
    ethereum::EthBlockHeader,
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

    pub fn run(self, call_data: CallTxData) {
        guest_evm_call(call_data, &self.env);
    }
}

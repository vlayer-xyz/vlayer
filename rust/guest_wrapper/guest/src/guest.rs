use vlayer_engine::{
    config::ETH_SEPOLIA_CHAIN_SPEC,
    contract::{db::WrapStateDb, engine::Engine},
    ethereum::EthBlockHeader,
    guest::{Call, Output},
    EvmEnv, EvmInput,
};

pub struct Guest {
    env: EvmEnv<WrapStateDb, EthBlockHeader>,
}

impl Guest {
    pub fn new(evm_input: EvmInput<EthBlockHeader>) -> Self {
        let env = evm_input
            .into_env()
            .with_chain_spec(&ETH_SEPOLIA_CHAIN_SPEC)
            .unwrap();

        Guest { env }
    }

    pub fn run(&mut self, call: Call) -> Output {
        Output {
            block_commitment: self.env.block_commitment(),
            evm_call_result: Engine::call::<EthBlockHeader>(&call, &mut self.env.db, &self.env.cfg_env, &self.env.header).unwrap(),
        }
    }
}

use vlayer_engine::{
    config::ETH_SEPOLIA_CHAIN_SPEC,
    contract::{db::WrapStateDb, engine::Engine},
    ethereum::EthBlockHeader,
    guest::{Call, Output},
    EvmBlockHeader, EvmInput,
};

pub struct Guest {
    db: WrapStateDb,
    header: EthBlockHeader,
}

impl Guest {
    pub fn new(evm_input: EvmInput<EthBlockHeader>) -> Self {
        let (db, header) = evm_input.into_db_and_header();
        Guest { db, header }
    }

    pub fn run(&mut self, call: Call) -> Output {
        Output {
            block_commitment: self.header.block_commitment(),
            evm_call_result: Engine::try_new(
                &mut self.db,
                self.header.clone(),
                &ETH_SEPOLIA_CHAIN_SPEC,
            )
            .unwrap()
            .call(&call)
            .unwrap(),
        }
    }
}

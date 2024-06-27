use vlayer_engine::{
    config::SEPOLIA_ID,
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
        let function_selector: [u8; 4] = call.data[0..4]
            .try_into()
            .expect("cannot extract function selector from call data");

        Output {
            execution_commitment: self.header.block_commitment(call.to, function_selector),

            evm_call_result: Engine::try_new(&mut self.db, self.header.clone(), SEPOLIA_ID)
                .unwrap()
                .call(&call)
                .unwrap(),
        }
    }
}

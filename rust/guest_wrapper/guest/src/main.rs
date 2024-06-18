#![no_main]

risc0_zkvm::guest::entry!(main);

use alloy_primitives::{address, Address};
use risc0_zkvm::guest::env;
use vlayer_steel::{
    config::ETH_SEPOLIA_CHAIN_SPEC, contract::{call::guest_evm_call, CallTxData}, ethereum::EthBlockHeader, guest_input::GuestInput, EvmEnv, EvmInput, StateDb
};

const CONTRACT: Address = address!("5fbdb2315678afecb367f032d93f642f64180aa3");

struct Guest {
    env: EvmEnv<StateDb, EthBlockHeader>
}

impl Guest {
    pub fn new(evm_input: EvmInput<EthBlockHeader>) -> Self {
        let env = evm_input
        .into_env()
        .with_chain_spec(&ETH_SEPOLIA_CHAIN_SPEC)
        .unwrap();

        Guest { env }
    }

    pub fn run(self, call_data: CallTxData<()>) {
        guest_evm_call(call_data, &self.env);
    }
}

fn main() {
    let GuestInput {
        evm_input,
        call_data,
    } = env::read();
    
    let call_data = CallTxData::<()>::new_from_bytes(CONTRACT, call_data);

    let returns = Guest::new(evm_input).run(call_data);
    env::commit(&returns);
}

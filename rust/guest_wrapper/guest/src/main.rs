#![no_main]

risc0_zkvm::guest::entry!(main);

use alloy_primitives::{address, Address};
use risc0_zkvm::guest::env;
use vlayer_steel::{
    config::ETH_SEPOLIA_CHAIN_SPEC,
    contract::{call::guest_evm_call, CallTxData},
    guest_input::GuestInput,
};

const CONTRACT: Address = address!("5fbdb2315678afecb367f032d93f642f64180aa3");

fn main() {
    let GuestInput {
        evm_input,
        call_data,
    } = env::read();

    let env = evm_input
        .into_env()
        .with_chain_spec(&ETH_SEPOLIA_CHAIN_SPEC)
        .unwrap();
    let call_data = CallTxData::<()>::new_from_bytes(CONTRACT, call_data);
    let returns = guest_evm_call(call_data, &env);

    env::commit(&returns);
}

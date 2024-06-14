#![no_main]

risc0_zkvm::guest::entry!(main);

use alloy_primitives::{address, Address, U256};
use alloy_sol_types::SolCall;
use risc0_zkvm::guest::env;
use vlayer_common::Simple::sumCall;
use vlayer_steel::{
    config::ETH_SEPOLIA_CHAIN_SPEC,
    contract::call_builder::{guest_evm_call, CallBuilder as SteelCallBuilder},
    ethereum::EthEvmInput,
};

const CONTRACT: Address = address!("5fbdb2315678afecb367f032d93f642f64180aa3");

fn main() {
    let input: EthEvmInput = env::read();
    let env = input.into_env().with_chain_spec(&ETH_SEPOLIA_CHAIN_SPEC);

    let call_data: Vec<u8> = env::read();
    let call = <sumCall as SolCall>::abi_decode(&call_data, true).unwrap();

    let call_builder = SteelCallBuilder::new(CONTRACT, &call);
    let returns = guest_evm_call(call_builder.into(), &env);

    assert!(returns._0 == U256::from(3));
}

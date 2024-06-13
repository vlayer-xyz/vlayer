#![no_main]

risc0_zkvm::guest::entry!(main);

use alloy_primitives::{address, Address, U256};
use alloy_sol_types::sol;
use risc0_zkvm::guest::env;
use vlayer_common::CallBuilder;
use vlayer_steel::{
    config::ETH_SEPOLIA_CHAIN_SPEC, contract::call_builder::guest_evm_call, ethereum::EthEvmInput,
    Contract,
};

sol! {
    interface Simple {
        function sum(uint256 lhs, uint256 rhs) public pure returns (uint256);
    }
}

const CONTRACT: Address = address!("5fbdb2315678afecb367f032d93f642f64180aa3");

fn main() {
    let input: EthEvmInput = env::read();
    let env = input.into_env().with_chain_spec(&ETH_SEPOLIA_CHAIN_SPEC);

    let call = CallBuilder::build();

    let call_builder = Contract::new(CONTRACT).call_builder(&call);
    let returns = guest_evm_call(call_builder, &env);

    assert!(returns._0 == U256::from(3));
}

#![no_main]

risc0_zkvm::guest::entry!(main);

use alloy_primitives::{address, Address, U256};
use alloy_sol_types::sol;
use risc0_steel::{config::ETH_SEPOLIA_CHAIN_SPEC, ethereum::EthEvmInput, Contract};
use risc0_zkvm::guest::env;

sol! {
    interface Simple {
        function sum(uint256 lhs, uint256 rhs) public pure returns (uint256);
    }
}

const CONTRACT: Address = address!("e7f1725e7734ce288f8367e1bb143e90bb3f0512");

fn main() {
    let input: EthEvmInput = env::read();
    let env = input.into_env().with_chain_spec(&ETH_SEPOLIA_CHAIN_SPEC);

    let lhs = U256::from(2);
    let rhs = U256::from(2);
    let call = Simple::sumCall { lhs, rhs };

    let returns = Contract::new(CONTRACT, &env).call_builder(&call).call();

    assert!(returns._0 == U256::from(4));
}

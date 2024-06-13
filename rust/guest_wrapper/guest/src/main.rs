#![no_main]

risc0_zkvm::guest::entry!(main);

use alloy_primitives::{address, Address, U256};
use alloy_sol_types::sol;
use risc0_zkvm::guest::env;
use vlayer_steel::{config::ETH_SEPOLIA_CHAIN_SPEC, ethereum::EthEvmInput, Contract};

sol! {
    interface Simple {
        function sum(uint256 lhs, uint256 rhs) public pure returns (uint256);
    }
}

const CONTRACT: Address = address!("5fbdb2315678afecb367f032d93f642f64180aa3");

fn main() {
    let input: EthEvmInput = env::read();
    let env = input.into_env().with_chain_spec(&ETH_SEPOLIA_CHAIN_SPEC);

    let lhs = U256::from(1);
    let rhs = U256::from(2);
    let call = Simple::sumCall { lhs, rhs };

    let returns = Contract::new(CONTRACT, &env).call_builder(&call).call();

    assert!(returns._0 == U256::from(3));
}

mod test {

    #[test]
    fn test() {
        assert!(1 == 1);
    }
}

use alloy_primitives::{address, Address, U256};
use alloy_sol_types::{sol, SolCall};

sol! {
  interface Simple {
      function sum(uint256 lhs, uint256 rhs) public pure returns (uint256);
  }
}

pub struct CallBuilder {}

impl CallBuilder {
    pub fn new() -> Self {
        Self {}
    }

    pub fn build() -> Simple::sumCall {
        let call: Simple::sumCall = Simple::sumCall {
            lhs: U256::from(1),
            rhs: U256::from(2),
        };
        call
    }
}

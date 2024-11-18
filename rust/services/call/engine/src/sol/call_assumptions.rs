// Keep everything in the vlayer library private except the assumptions.
mod private {
    alloy_sol_types::sol!(#![sol(all_derives)] "../../../../contracts/src/CallAssumptions.sol");
}

use alloy_primitives::{Address, U256};
use alloy_sol_types::SolType;
use block_header::EvmBlockHeader;
/// Solidity struct representing the committed block used for validation.
pub use private::CallAssumptions;

use crate::io::CallSelector;

impl CallAssumptions {
    /// Returns the [SolAssumptions] used to validate the environment.
    pub fn new(header: &dyn EvmBlockHeader, to: Address, selector: CallSelector) -> Self {
        Self {
            proverContractAddress: to,
            functionSelector: selector.into(),
            settleBlockHash: header.hash_slow(),
            settleBlockNumber: U256::from(header.number()),
        }
    }

    pub fn size() -> usize {
        Self::ENCODED_SIZE.expect("CallAssumptions compiletime size does not exist")
    }
}

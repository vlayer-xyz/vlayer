// Keep everything in the vlayer library private except the commitment.
mod private {
    alloy_sol_types::sol!(#![sol(all_derives)] "../../../contracts/src/ExecutionCommitment.sol");
}

use alloy_primitives::{Address, U256};
use alloy_sol_types::SolType;
/// Solidity struct representing the committed block used for validation.
pub use private::ExecutionCommitment;

use crate::{block_header::EvmBlockHeader, io::CallSelector};

impl ExecutionCommitment {
    /// Returns the [SolCommitment] used to validate the environment.
    pub fn new(header: &dyn EvmBlockHeader, to: Address, selector: CallSelector) -> Self {
        Self {
            proverContractAddress: to,
            functionSelector: selector.into(),
            settleBlockHash: header.hash_slow(),
            settleBlockNumber: U256::from(header.number()),
        }
    }

    pub fn size() -> usize {
        Self::ENCODED_SIZE.expect("ExecutionCommitment compiletime size does not exist")
    }
}

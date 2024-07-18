// Keep everything in the Vlayer library private except the commitment.
mod private {
    alloy_sol_types::sol!("../../contracts/src/ExecutionCommitment.sol");
}

use alloy_primitives::{Address, U256};
/// Solidity struct representing the committed block used for validation.
pub use private::ExecutionCommitment;

use crate::{
    block_header::evm::{EvmBlockHeader, Hashable},
    io::CallSelector,
};

impl ExecutionCommitment {
    /// Returns the [SolCommitment] used to validate the environment.
    pub fn new<H: EvmBlockHeader + Hashable + Clone>(
        header: &H,
        to: Address,
        selector: CallSelector,
    ) -> Self {
        Self {
            startContractAddress: to,
            functionSelector: selector.into(),
            settleBlockHash: header.hash_slow(),
            settleBlockNumber: U256::from(header.number()),
        }
    }
}

// Keep everything in the Vlayer library private except the commitment.
mod private {
    alloy_sol_types::sol!("./Vlayer.sol");
}

use alloy_primitives::{Address, Sealable, U256};
/// Solidity struct representing the committed block used for validation.
pub use private::Vlayer::ExecutionCommitment;

use crate::{evm::block_header::EvmBlockHeader, io::CallSelector};

impl ExecutionCommitment {
    /// Returns the [SolCommitment] used to validate the environment.
    pub fn new<H: EvmBlockHeader + Sealable + Clone>(
        header: &H,
        to: Address,
        selector: CallSelector,
    ) -> Self {
        Self {
            startContractAddress: to,
            functionSelector: selector.into(),
            settleBlockHash: header.clone().seal_slow().seal(),
            settleBlockNumber: U256::from(header.number()),
        }
    }
}

use alloy_primitives::{Address, BlockNumber, Sealable, B256};

use revm::primitives::BlockEnv;

use crate::ExecutionCommitment;

/// An EVM abstraction of a block header.
pub trait EvmBlockHeader: Sealable {
    /// Returns the hash of the parent block's header.
    fn parent_hash(&self) -> &B256;
    /// Returns the block number.
    fn number(&self) -> BlockNumber;
    /// Returns the block timestamp.
    fn timestamp(&self) -> u64;
    /// Returns the state root hash.
    fn state_root(&self) -> &B256;

    /// Returns the [ExecutionCommitment] used to validate the environment.
    fn execution_commitment(
        &self,
        start_contract_address: Address,
        function_selector: [u8; 4],
    ) -> ExecutionCommitment;
    /// Fills the EVM block environment with the header's data.
    fn fill_block_env(&self, blk_env: &mut BlockEnv);
}

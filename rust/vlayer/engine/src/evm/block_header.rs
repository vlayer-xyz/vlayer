use alloy_primitives::{BlockNumber, B256};

use revm::primitives::BlockEnv;

pub trait Hashable {
    /// Calculate the hash, this may be slow.
    fn hash_slow(&self) -> B256;
}

/// An EVM abstraction of a block header.
pub trait EvmBlockHeader: Hashable {
    /// Returns the hash of the parent block's header.
    fn parent_hash(&self) -> &B256;
    /// Returns the block number.
    fn number(&self) -> BlockNumber;
    /// Returns the block timestamp.
    fn timestamp(&self) -> u64;
    /// Returns the state root hash.
    fn state_root(&self) -> &B256;
    /// Fills the EVM block environment with the header's data.
    fn fill_block_env(&self, blk_env: &mut BlockEnv);
}

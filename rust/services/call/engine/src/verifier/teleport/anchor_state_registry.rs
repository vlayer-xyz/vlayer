use alloy_primitives::{Address, B256, U256};
use anyhow::anyhow;
use derive_new::new;
use revm::DatabaseRef;

use super::{Error, Result};

/// Storage layout:
/// `OutputRoot` struct is stored at mapping(GameType -> OutputRoot) in slot 1.
/// - Field 0: `hash` (bytes32) -> Located at keccak(GameType . 1)
/// - Field 1: `blockNumber` (uint256) -> Located at keccak(GameType . 1) + 1
mod layout {
    use alloy_primitives::U256;
    use lazy_static::lazy_static;
    lazy_static! {
        pub static ref OUTPUT_HASH_SLOT: U256 = U256::from_str_radix(
            "a6eef7e35abe7026729641147f7915573c7e97b47efa546f5f6e3230263bcb49",
            16
        )
        .unwrap();
        pub static ref BLOCK_NUMBER_SLOT: U256 = *OUTPUT_HASH_SLOT + U256::from(1);
    }
}

#[derive(Clone, Debug)]
pub struct L2Commitment {
    pub output_hash: B256,
    pub block_number: U256,
}

#[derive(Clone, Debug, new)]
pub struct AnchorStateRegistry {
    address: Address,
}

impl AnchorStateRegistry {
    pub fn get_latest_confirmed_l2_commitment<D>(&self, db: D) -> Result<L2Commitment>
    where
        D: DatabaseRef + Send + Sync,
        D::Error: std::fmt::Debug + std::error::Error + Send + Sync + 'static,
    {
        let root = db
            .storage_ref(self.address, *layout::OUTPUT_HASH_SLOT)
            .map_err(|err| Error::Database(anyhow!(err)))?;
        let block_number = db
            .storage_ref(self.address, *layout::BLOCK_NUMBER_SLOT)
            .map_err(|err| Error::Database(anyhow!(err)))?;

        Ok(L2Commitment {
            output_hash: B256::from(root),
            block_number,
        })
    }
}

use alloy_primitives::{Address, B256, U256};
use anyhow::anyhow;
use derive_new::new;
use lazy_static::lazy_static;
use revm::DatabaseRef;

use super::{Error, Result};

lazy_static! {
    // https://etherscan.deth.net/address/0x18DAc71c228D1C32c99489B7323d441E1175e443#readProxyContract
    // mapping: GameType -> OutputRoot
    // struct OutputRoot {
    //     hash: bytes32,
    //     blockNumber: uint256,
    // }
    static ref OUTPUT_HASH_SLOT: U256 = U256::from_str_radix(
        // keccak(key . position).
        // Key = game type = 0
        // Position = 1
        "a6eef7e35abe7026729641147f7915573c7e97b47efa546f5f6e3230263bcb4a",
        16
    )
    .unwrap();
    // Second field of a struct
    static ref BLOCK_NUMBER_SLOT: U256 = *OUTPUT_HASH_SLOT + U256::from(1);
}

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
            .storage_ref(self.address, *OUTPUT_HASH_SLOT)
            .map_err(|err| Error::Database(anyhow!(err)))?;
        let block_number = db
            .storage_ref(self.address, *BLOCK_NUMBER_SLOT)
            .map_err(|err| Error::Database(anyhow!(err)))?;

        Ok(L2Commitment {
            output_hash: B256::from(root),
            block_number,
        })
    }
}

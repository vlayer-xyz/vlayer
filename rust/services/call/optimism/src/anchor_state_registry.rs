use alloy_primitives::{Address, B256, BlockNumber};
use anyhow::anyhow;
use call_common::RevmDB;
use derive_new::new;

#[derive(thiserror::Error, Debug)]
#[error(transparent)]
pub struct Error(#[from] anyhow::Error);
type Result<T> = std::result::Result<T, Error>;

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
    pub block_number: BlockNumber,
}

#[derive(Clone, Debug, new)]
pub struct AnchorStateRegistry<D: RevmDB> {
    address: Address,
    db: D,
}

impl<D: RevmDB> AnchorStateRegistry<D> {
    pub fn get_latest_confirmed_l2_commitment(&self) -> Result<L2Commitment> {
        // `WrapStateDB` relies on the guarantee that EVM always asks for account state before storage and caches some things
        // Therefore - without this step - we can't access storage
        let _ = self
            .db
            .basic_ref(self.address)
            .map_err(|err| anyhow!(err))?;
        let root = self
            .db
            .storage_ref(self.address, *layout::OUTPUT_HASH_SLOT)
            .map_err(|err| anyhow!(err))?;
        let block_number = self
            .db
            .storage_ref(self.address, *layout::BLOCK_NUMBER_SLOT)
            .map_err(|err| anyhow!(err))?;

        Ok(L2Commitment {
            output_hash: B256::from(root),
            block_number: block_number.to::<BlockNumber>(),
        })
    }
}

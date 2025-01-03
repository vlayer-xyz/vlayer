use alloy_primitives::{Address, BlockNumber, Bytes, StorageKey, StorageValue, TxNumber, U256};
use block_header::EvmBlockHeader;
use ethers_core::types::BlockNumber as BlockTag;

use super::{BlockingProvider, EIP1186Proof, Result};

/// Provider that always returns default values. Useful for testing other providers
#[derive(Debug, PartialEq)]
pub struct DefaultProvider;

impl BlockingProvider for DefaultProvider {
    fn get_block_header(&self, _: BlockTag) -> Result<Option<Box<dyn EvmBlockHeader>>> {
        Ok(Default::default())
    }

    fn get_transaction_count(&self, _: Address, _: BlockNumber) -> Result<TxNumber> {
        Ok(Default::default())
    }

    fn get_balance(&self, _: Address, _: BlockNumber) -> Result<U256> {
        Ok(Default::default())
    }

    fn get_code(&self, _: Address, _: BlockNumber) -> Result<Bytes> {
        Ok(Default::default())
    }

    fn get_storage_at(&self, _: Address, _: StorageKey, _: BlockNumber) -> Result<StorageValue> {
        Ok(Default::default())
    }

    fn get_proof(&self, _: Address, _: Vec<StorageKey>, _: BlockNumber) -> Result<EIP1186Proof> {
        Ok(Default::default())
    }

    fn get_latest_block_number(&self) -> Result<BlockNumber> {
        Ok(Default::default())
    }
}

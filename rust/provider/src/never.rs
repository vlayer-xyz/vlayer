use std::marker::PhantomData;

use alloy_primitives::{Address, BlockNumber, Bytes, StorageKey, StorageValue, TxNumber, U256};
use block_header::EvmBlockHeader;
use ethers_core::types::BlockNumber as BlockTag;

use super::{BlockingProvider, EIP1186Proof, Result};

/// A simple provider that panics on all queries.
#[derive(Debug, PartialEq)]
pub struct NeverProvider(pub PhantomData<Box<dyn EvmBlockHeader>>);

#[allow(clippy::panic)]
impl BlockingProvider for NeverProvider {
    fn get_block_header(&self, _: BlockTag) -> Result<Option<Box<dyn EvmBlockHeader>>> {
        panic!("Unexpected provider call")
    }

    fn get_transaction_count(&self, _: Address, _: BlockNumber) -> Result<TxNumber> {
        panic!("Unexpected provider call")
    }

    fn get_balance(&self, _: Address, _: BlockNumber) -> Result<U256> {
        panic!("Unexpected provider call")
    }

    fn get_code(&self, _: Address, _: BlockNumber) -> Result<Bytes> {
        panic!("Unexpected provider call")
    }

    fn get_storage_at(&self, _: Address, _: StorageKey, _: BlockNumber) -> Result<StorageValue> {
        panic!("Unexpected provider call")
    }

    fn get_proof(&self, _: Address, _: Vec<StorageKey>, _: BlockNumber) -> Result<EIP1186Proof> {
        panic!("Unexpected provider call")
    }

    fn get_latest_block_number(&self) -> Result<BlockNumber> {
        panic!("Unexpected provider call")
    }
}

use super::{BlockingProvider, EIP1186Proof};
use alloy_primitives::{Address, Bytes, StorageKey, StorageValue, TxNumber, U256};
use std::{convert::Infallible, marker::PhantomData};
use vlayer_engine::block_header::EvmBlockHeader;

/// A simple provider that panics on all queries.
#[derive(Debug, PartialEq)]
pub struct NullProvider(pub(crate) PhantomData<Box<dyn EvmBlockHeader>>);

impl BlockingProvider for NullProvider {
    type Error = Infallible;

    fn get_block_header(
        &self,
        _: ethers_core::types::BlockNumber,
    ) -> Result<Option<Box<dyn EvmBlockHeader>>, Self::Error> {
        panic!("Unexpected provider call")
    }
    fn get_transaction_count(
        &self,
        _: Address,
        _: alloy_primitives::BlockNumber,
    ) -> Result<TxNumber, Self::Error> {
        panic!("Unexpected provider call")
    }
    fn get_balance(
        &self,
        _: Address,
        _: alloy_primitives::BlockNumber,
    ) -> Result<U256, Self::Error> {
        panic!("Unexpected provider call")
    }
    fn get_code(&self, _: Address, _: alloy_primitives::BlockNumber) -> Result<Bytes, Self::Error> {
        panic!("Unexpected provider call")
    }
    fn get_storage_at(
        &self,
        _: Address,
        _: StorageKey,
        _: alloy_primitives::BlockNumber,
    ) -> Result<StorageValue, Self::Error> {
        panic!("Unexpected provider call")
    }
    fn get_proof(
        &self,
        _: Address,
        _: Vec<StorageKey>,
        _: alloy_primitives::BlockNumber,
    ) -> Result<EIP1186Proof, Self::Error> {
        panic!("Unexpected provider call")
    }
}

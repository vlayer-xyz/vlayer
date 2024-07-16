use super::{EIP1186Proof, Provider};
use alloy_primitives::{Address, BlockNumber, Bytes, StorageKey, StorageValue, TxNumber, U256};
use std::{convert::Infallible, marker::PhantomData};
use vlayer_engine::evm::block_header::EvmBlockHeader;

/// A simple provider that panics on all queries.
pub struct NullProvider(pub(crate) PhantomData<Box<dyn EvmBlockHeader>>);

impl Provider for NullProvider {
    type Error = Infallible;

    fn get_block_header(
        &self,
        _: BlockNumber,
    ) -> Result<Option<Box<dyn EvmBlockHeader>>, Self::Error> {
        panic!("Unexpected provider call")
    }
    fn get_transaction_count(&self, _: Address, _: BlockNumber) -> Result<TxNumber, Self::Error> {
        panic!("Unexpected provider call")
    }
    fn get_balance(&self, _: Address, _: BlockNumber) -> Result<U256, Self::Error> {
        panic!("Unexpected provider call")
    }
    fn get_code(&self, _: Address, _: BlockNumber) -> Result<Bytes, Self::Error> {
        panic!("Unexpected provider call")
    }
    fn get_storage_at(
        &self,
        _: Address,
        _: StorageKey,
        _: BlockNumber,
    ) -> Result<StorageValue, Self::Error> {
        panic!("Unexpected provider call")
    }
    fn get_proof(
        &self,
        _: Address,
        _: Vec<StorageKey>,
        _: BlockNumber,
    ) -> Result<EIP1186Proof, Self::Error> {
        panic!("Unexpected provider call")
    }
}

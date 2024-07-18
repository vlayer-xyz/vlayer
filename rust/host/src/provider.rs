use alloy_primitives::{Address, BlockNumber, Bytes, StorageKey, StorageValue, TxNumber, U256};
use auto_impl::auto_impl;
use std::error::Error as StdError;
use vlayer_engine::block_header::EvmBlockHeader;

mod cache;
pub mod ethers;
pub mod factory;
mod file;
pub mod multi;
mod null;

pub use ethers::{EthersProvider, EthersProviderError};
use ethers_providers::{Http, RetryClient};
pub use file::{EthFileProvider, FileProvider};

use crate::proof::EIP1186Proof;

/// The Ethers client type.
pub type EthersClient = ethers_providers::Provider<RetryClient<Http>>;

/// A trait for providers that fetch data from the Ethereum blockchain.
#[auto_impl(Rc)]
pub trait Provider {
    type Error: StdError + Send + Sync + 'static;
    type Header: EvmBlockHeader;

    fn get_block_header(&self, block: BlockNumber) -> Result<Option<Self::Header>, Self::Error>;
    fn get_transaction_count(
        &self,
        address: Address,
        block: BlockNumber,
    ) -> Result<TxNumber, Self::Error>;
    fn get_balance(&self, address: Address, block: BlockNumber) -> Result<U256, Self::Error>;
    fn get_code(&self, address: Address, block: BlockNumber) -> Result<Bytes, Self::Error>;
    fn get_storage_at(
        &self,
        address: Address,
        key: StorageKey,
        block: BlockNumber,
    ) -> Result<StorageValue, Self::Error>;
    fn get_proof(
        &self,
        address: Address,
        storage_keys: Vec<StorageKey>,
        block: BlockNumber,
    ) -> Result<EIP1186Proof, Self::Error>;
}

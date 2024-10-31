mod cache;
mod ethers;
mod factory;
mod multi;
mod null;
mod proof;

use std::fmt::Debug;

pub use alloy_primitives::{Address, BlockNumber, Bytes, StorageKey, StorageValue, TxNumber, U256};
use anyhow::Result;
use auto_impl::auto_impl;
pub use block_header::EvmBlockHeader;
pub use ethers::*;
pub use ethers_core::types::BlockNumber as BlockTag;
use ethers_providers::{Http, RetryClient};
pub use factory::{
    CachedProviderFactory, EthersProviderFactory, ProviderFactory, ProviderFactoryError,
};
pub use multi::CachedMultiProvider;
pub use proof::{EIP1186Proof, StorageProof};

/// The Ethers client type.
pub type EthersClient = ethers_providers::Provider<RetryClient<Http>>;

/// A trait for providers that fetch data from the Ethereum blockchain.
#[auto_impl(Rc, Box)]
pub trait BlockingProvider: Debug + Send + Sync {
    fn get_balance(&self, address: Address, block: BlockNumber) -> Result<U256>;
    fn get_block_header(&self, block: BlockTag) -> Result<Option<Box<dyn EvmBlockHeader>>>;
    fn get_code(&self, address: Address, block: BlockNumber) -> Result<Bytes>;
    fn get_proof(
        &self,
        address: Address,
        storage_keys: Vec<StorageKey>,
        block: BlockNumber,
    ) -> Result<EIP1186Proof>;
    fn get_storage_at(
        &self,
        address: Address,
        key: StorageKey,
        block: BlockNumber,
    ) -> Result<StorageValue>;
    fn get_transaction_count(&self, address: Address, block: BlockNumber) -> Result<TxNumber>;
}

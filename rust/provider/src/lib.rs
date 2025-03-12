mod cache;
mod default;
mod ethers;
pub mod factory;
pub mod multi;
pub mod never;
pub mod profiling;
mod proof;
mod provider_ext;

use std::fmt::Debug;

pub use alloy_primitives::{Address, BlockNumber, Bytes, StorageKey, StorageValue, TxNumber, U256};
use auto_impl::auto_impl;
pub use block_header::EvmBlockHeader;
pub use cache::CachedProvider;
pub use ethers::*;
pub use ethers_core::types::BlockNumber as BlockTag;
use ethers_providers::{Http, RetryClient};
pub use factory::{
    CachedProviderFactory, EthersProviderFactory, NullProviderFactory, ProviderFactory,
};
pub use multi::CachedMultiProvider;
pub use proof::{EIP1186Proof, StorageProof};
pub use provider_ext::BlockingProviderExt;
use thiserror::Error;

/// The Ethers client type.
pub type EthersClient = ethers_providers::Provider<RetryClient<Http>>;

#[derive(Debug, Error)]
pub enum Error {
    #[error("Provider: {0}")]
    Opaque(#[from] anyhow::Error),
    #[error("Ethers provider: {0}")]
    Ethers(#[from] ethers_providers::ProviderError),
}
pub type Result<T> = std::result::Result<T, Error>;

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
    fn get_latest_block_number(&self) -> Result<BlockNumber>;
}

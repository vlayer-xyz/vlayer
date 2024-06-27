use alloy_primitives::{
    Address, BlockNumber, Bytes, StorageKey, StorageValue, TxNumber, B256, U256,
};
use serde::{Deserialize, Serialize};
use std::{convert::Infallible, error::Error as StdError, fmt::Debug, marker::PhantomData};
use vlayer_engine::evm::block_header::EvmBlockHeader;

mod ethers;
mod file;

pub use ethers::{EthersProvider, EthersProviderError};
use ethers_providers::{Http, RetryClient};
pub use file::{CachedProvider, EthFileProvider, FileProvider};

/// The Ethers client type.
pub type EthersClient = ethers_providers::Provider<RetryClient<Http>>;

/// A trait for providers that fetch data from the Ethereum blockchain.
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

/// Data structure with proof for one single storage-entry
#[derive(Debug, Default, Clone, PartialEq, Eq, Deserialize, Serialize)]
pub struct StorageProof {
    pub key: StorageKey,
    pub proof: Vec<Bytes>,
    pub value: StorageValue,
}

/// Response for EIP-1186 account proof `eth_getProof`
#[derive(Debug, Default, Clone, PartialEq, Eq, Deserialize, Serialize)]
pub struct EIP1186Proof {
    pub address: Address,
    pub balance: U256,
    pub code_hash: B256,
    pub nonce: TxNumber,
    pub storage_hash: B256,
    pub account_proof: Vec<Bytes>,
    pub storage_proof: Vec<StorageProof>,
}

/// A simple provider that panics on all queries.
pub struct NullProvider<H>(PhantomData<H>);

impl<H: EvmBlockHeader> Provider for NullProvider<H> {
    type Error = Infallible;
    type Header = H;

    fn get_block_header(&self, _: BlockNumber) -> Result<Option<Self::Header>, Self::Error> {
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

use alloy_primitives::{
    Address, BlockNumber, Bytes, ChainId, StorageKey, StorageValue, TxNumber, B256, U256,
};
use auto_impl::auto_impl;
use ethers::{from_ethers_bytes, from_ethers_u256};
use ethers_core::types::StorageProof as EthersStorageProof;
use factory::ProviderFactory;
use serde::{Deserialize, Serialize};
use std::{collections::HashMap, error::Error as StdError, fmt::Debug, rc::Rc};
use vlayer_engine::evm::block_header::EvmBlockHeader;

mod cache;
mod ethers;
pub mod factory;
mod file;
mod null;

pub use ethers::{EthersProvider, EthersProviderError};
use ethers_providers::{Http, RetryClient};
pub use file::{EthFileProvider, FileProvider};

use crate::{host::error::HostError, utils::get_mut_or_insert_with_result};

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

type MultiProvider<P> = HashMap<ChainId, Rc<P>>;

pub struct CachedMultiProvider<P> {
    cache: MultiProvider<P>,
    factory: Box<dyn ProviderFactory<P>>,
}

impl<P> CachedMultiProvider<P>
where
    P: Provider,
{
    pub fn new(factory: impl ProviderFactory<P> + 'static) -> Self {
        CachedMultiProvider {
            cache: HashMap::new(),
            factory: Box::new(factory),
        }
    }

    pub fn try_get(&mut self, chain_id: ChainId) -> Result<Rc<P>, HostError> {
        let create_provider = || Ok::<_, HostError>(Rc::new(self.factory.create(chain_id)?));
        Ok(Rc::clone(get_mut_or_insert_with_result(
            &mut self.cache,
            chain_id,
            create_provider,
        )?))
    }
}

/// Data structure with proof for one single storage-entry
#[derive(Debug, Default, Clone, PartialEq, Eq, Deserialize, Serialize)]
pub struct StorageProof {
    pub key: StorageKey,
    pub proof: Vec<Bytes>,
    pub value: StorageValue,
}

impl From<EthersStorageProof> for StorageProof {
    fn from(proof: EthersStorageProof) -> Self {
        StorageProof {
            key: from_ethers_u256(proof.key).to_be_bytes().into(),
            proof: proof.proof.into_iter().map(from_ethers_bytes).collect(),
            value: from_ethers_u256(proof.value),
        }
    }
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

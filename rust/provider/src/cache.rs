use std::{collections::hash_map::Entry, path::PathBuf, sync::RwLock};

use alloy_primitives::{Address, BlockNumber, Bytes, StorageKey, StorageValue, TxNumber, U256};
use anyhow::bail;
use block_header::EvmBlockHeader;
use derivative::Derivative;
use ethers_core::types::BlockNumber as BlockTag;
use json::{AccountQuery, BlockQuery, JsonCache, ProofQuery, StorageQuery};

use super::{BlockingProvider, EIP1186Proof};

pub(crate) mod json;

/// A provider that caches responses from an underlying provider in a JSON file.
/// Queries are first checked against the cache, and if not found, the provider is invoked.
/// The cache is saved when the provider is dropped.
#[derive(Debug, Derivative)]
#[derivative(PartialEq)]
pub struct CachedProvider<P: BlockingProvider> {
    pub(super) inner: P,
    #[derivative(PartialEq = "ignore")]
    pub(super) cache: RwLock<JsonCache>,
}

impl<P: BlockingProvider> CachedProvider<P> {
    /// Creates a new [CachedProvider]. At this point, the cache files
    /// directory should exist and the cache file itself should not.
    /// A new cache file will be created when dropped.
    pub fn new(cache_path: PathBuf, provider: P) -> anyhow::Result<Self> {
        // Sanity checks.
        if let Some(parent) = cache_path.parent() {
            if !parent.exists() {
                bail!("Cache files directory '{}' does not exist.", parent.display());
            }
        }
        if cache_path.exists() {
            bail!(
                "Cache file {} already exists. Are you trying to create two test files with the same name?",
                cache_path.display()
            );
        }

        let cache = JsonCache::empty(cache_path);
        Ok(Self {
            inner: provider,
            cache: RwLock::new(cache),
        })
    }
}

impl<P: BlockingProvider> BlockingProvider for CachedProvider<P> {
    type Error = P::Error;

    fn get_block_header(
        &self,
        block: BlockTag,
    ) -> Result<Option<Box<dyn EvmBlockHeader>>, Self::Error> {
        match self
            .cache
            .write()
            .expect("poisoned RwLock")
            .partial_blocks
            .entry(BlockQuery {
                block_no: block.into(),
            }) {
            Entry::Occupied(entry) => Ok(entry.get().clone()),
            Entry::Vacant(v) => match self.inner.get_block_header(block) {
                Ok(header) => Ok(v.insert(header).clone()),
                Err(e) => Err(e),
            },
        }
    }

    fn get_transaction_count(
        &self,
        address: Address,
        block: BlockNumber,
    ) -> Result<TxNumber, Self::Error> {
        match self
            .cache
            .write()
            .expect("poisoned RwLock")
            .transaction_count
            .entry(AccountQuery {
                block_no: block,
                address,
            }) {
            Entry::Occupied(entry) => Ok(*entry.get()),
            Entry::Vacant(entry) => {
                let count = self.inner.get_transaction_count(address, block)?;
                Ok(*entry.insert(count))
            }
        }
    }

    fn get_balance(&self, address: Address, block: BlockNumber) -> Result<U256, Self::Error> {
        match self
            .cache
            .write()
            .expect("poisoned RwLock")
            .balance
            .entry(AccountQuery {
                block_no: block,
                address,
            }) {
            Entry::Occupied(entry) => Ok(*entry.get()),
            Entry::Vacant(entry) => {
                let balance = self.inner.get_balance(address, block)?;
                Ok(*entry.insert(balance))
            }
        }
    }

    fn get_code(&self, address: Address, block: BlockNumber) -> Result<Bytes, Self::Error> {
        match self
            .cache
            .write()
            .expect("poisoned RwLock")
            .code
            .entry(AccountQuery {
                block_no: block,
                address,
            }) {
            Entry::Occupied(entry) => Ok(entry.get().clone()),
            Entry::Vacant(entry) => {
                let code = self.inner.get_code(address, block)?;
                Ok(entry.insert(code).clone())
            }
        }
    }

    fn get_storage_at(
        &self,
        address: Address,
        key: StorageKey,
        block: BlockNumber,
    ) -> Result<StorageValue, Self::Error> {
        match self
            .cache
            .write()
            .expect("poisoned RwLock")
            .storage
            .entry(StorageQuery {
                block_no: block,
                address,
                key,
            }) {
            Entry::Occupied(entry) => Ok(*entry.get()),
            Entry::Vacant(entry) => {
                let storage = self.inner.get_storage_at(address, key, block)?;
                Ok(*entry.insert(storage))
            }
        }
    }

    fn get_proof(
        &self,
        address: Address,
        storage_keys: Vec<StorageKey>,
        block: BlockNumber,
    ) -> Result<EIP1186Proof, Self::Error> {
        match self
            .cache
            .write()
            .expect("poisoned RwLock")
            .proofs
            .entry(ProofQuery {
                block_no: block,
                address,
                storage_keys: storage_keys.iter().cloned().collect(),
            }) {
            Entry::Occupied(entry) => Ok(entry.get().clone()),
            Entry::Vacant(entry) => {
                let proof = self.inner.get_proof(address, storage_keys, block)?;
                Ok(entry.insert(proof).clone())
            }
        }
    }
}

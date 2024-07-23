use super::{BlockingProvider, EIP1186Proof};
use alloy_primitives::{Address, BlockNumber, Bytes, StorageKey, StorageValue, TxNumber, U256};
use anyhow::Context;
use json::{AccountQuery, BlockQuery, JsonCache, ProofQuery, StorageQuery};
use std::{
    cell::RefCell,
    collections::hash_map::Entry,
    fs::{self},
    path::PathBuf,
};
use vlayer_engine::block_header::EvmBlockHeader;

pub mod json;

/// A provider that caches responses from an underlying provider in a JSON file.
/// Queries are first checked against the cache, and if not found, the provider is invoked.
/// The cache is saved when the provider is dropped.
#[derive(Debug, PartialEq)]
pub struct CachedProvider<P: BlockingProvider> {
    pub(super) inner: P,
    pub(super) cache: RefCell<JsonCache>,
}

impl<P: BlockingProvider> CachedProvider<P> {
    /// Creates a new [CachedProvider]. If the cache file exists, it will be read and deserialized.
    /// Otherwise, a new file will be created when dropped.
    pub fn new(cache_path: PathBuf, provider: P) -> anyhow::Result<Self> {
        let cache = match JsonCache::<P::Header>::from_file(cache_path.clone()) {
            Ok(_) => {
                fs::remove_file(&cache_path)?;
                JsonCache::empty(cache_path)
            },
            Err(err) => match err.downcast_ref::<std::io::Error>() {
                Some(io_err) if io_err.kind() == std::io::ErrorKind::NotFound => {
                    // create the file and directory if it doesn't exist
                    if let Some(parent) = cache_path.parent() {
                        fs::create_dir_all(parent).context("failed to create directory")?;
                    }
                    JsonCache::empty(cache_path)
                }
                _ => return Err(err),
            },
        };

        Ok(Self {
            inner: provider,
            cache: RefCell::new(cache),
        })
    }
}

impl<P: BlockingProvider> BlockingProvider for CachedProvider<P> {
    type Error = P::Error;

    fn get_block_header(
        &self,
        block: BlockNumber,
    ) -> Result<Option<Box<dyn EvmBlockHeader>>, Self::Error> {
        match self
            .cache
            .borrow_mut()
            .partial_blocks
            .entry(BlockQuery { block_no: block })
        {
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
            .borrow_mut()
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
        match self.cache.borrow_mut().balance.entry(AccountQuery {
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
        match self.cache.borrow_mut().code.entry(AccountQuery {
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
        match self.cache.borrow_mut().storage.entry(StorageQuery {
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
        match self.cache.borrow_mut().proofs.entry(ProofQuery {
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

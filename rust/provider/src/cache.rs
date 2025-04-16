use std::{
    collections::hash_map::Entry,
    fs::{create_dir_all, remove_file},
    marker::PhantomData,
    path::{Path, PathBuf},
    sync::RwLock,
};

use alloy_primitives::{Address, BlockNumber, Bytes, StorageKey, StorageValue, TxNumber, U256};
use block_header::EvmBlockHeader;
use ethers_core::types::BlockNumber as BlockTag;
use json::{AccountQuery, BlockQuery, JsonCache, ProofQuery, StorageQuery};

use super::{BlockingProvider, EIP1186Proof, Result};
use crate::never::NeverProvider;

pub(crate) mod json;

/// A provider that caches responses from an underlying provider in a JSON file.
///
/// Queries are first checked against the cache, and if not found, the provider is invoked.
/// The cache is saved when the provider is dropped.
#[derive(Debug)]
pub struct CachedProvider {
    pub(super) inner: Box<dyn BlockingProvider>,
    pub(super) cache: RwLock<JsonCache>,
}

fn ensure_parent_directory_exists(cache_path: &Path) -> anyhow::Result<()> {
    if let Some(parent) = cache_path.parent() {
        if !parent.exists() {
            println!("Parent directory '{}' does not exist. Creating it...", parent.display());
            create_dir_all(parent)?;
        }
    }
    Ok(())
}

fn remove_existing_cache_file(cache_path: &Path) -> anyhow::Result<()> {
    if cache_path.exists() {
        println!("Cache file '{}' already exists. Removing it...", cache_path.display());
        remove_file(cache_path)?;
    }
    Ok(())
}

impl CachedProvider {
    /// Creates a new [CachedProvider]
    /// A new cache file will be created when dropped.
    pub fn new(cache_path: PathBuf, provider: impl BlockingProvider + 'static) -> Result<Self> {
        ensure_parent_directory_exists(&cache_path)?;
        remove_existing_cache_file(&cache_path)?;

        let cache = JsonCache::empty(cache_path);
        Ok(Self::from_components(cache, provider))
    }

    pub fn from_file(file_path: &PathBuf) -> Result<Self> {
        let cache = JsonCache::load(file_path)?;
        Ok(Self::from_components(cache, NeverProvider(PhantomData)))
    }

    fn from_components(cache: JsonCache, provider: impl BlockingProvider + 'static) -> Self {
        Self {
            inner: Box::new(provider),
            cache: RwLock::new(cache),
        }
    }
}

#[allow(clippy::expect_used)]
impl BlockingProvider for CachedProvider {
    fn get_block_header(&self, block: BlockTag) -> Result<Option<Box<dyn EvmBlockHeader>>> {
        let mut cache = self.cache.write().expect("poisoned RwLock");
        match cache.partial_blocks.entry(BlockQuery {
            block_no: block.into(),
        }) {
            Entry::Occupied(entry) => Ok(entry.get().clone()),
            Entry::Vacant(v) => match self.inner.get_block_header(block) {
                Ok(header) => Ok(v.insert(header).clone()),
                Err(e) => Err(e),
            },
        }
    }

    fn get_transaction_count(&self, address: Address, block: BlockNumber) -> Result<TxNumber> {
        let mut cache = self.cache.write().expect("poisoned RwLock");
        match cache.transaction_count.entry(AccountQuery {
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

    fn get_balance(&self, address: Address, block: BlockNumber) -> Result<U256> {
        let mut cache = self.cache.write().expect("poisoned RwLock");
        match cache.balance.entry(AccountQuery {
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

    fn get_code(&self, address: Address, block: BlockNumber) -> Result<Bytes> {
        let mut cache = self.cache.write().expect("poisoned RwLock");
        match cache.code.entry(AccountQuery {
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
    ) -> Result<StorageValue> {
        let mut cache = self.cache.write().expect("poisoned RwLock");
        match cache.storage.entry(StorageQuery {
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
    ) -> Result<EIP1186Proof> {
        let mut cache = self.cache.write().expect("poisoned RwLock");
        match cache.proofs.entry(ProofQuery {
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

    fn get_latest_block_number(&self) -> Result<BlockNumber> {
        todo!()
    }
}

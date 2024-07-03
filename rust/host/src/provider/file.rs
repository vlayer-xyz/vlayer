use super::{cache::CachedProvider, null::NullProvider};
use crate::provider::cache::json::JsonCache;
use serde::{de::DeserializeOwned, Serialize};
use std::{cell::RefCell, marker::PhantomData, path::PathBuf};
use vlayer_engine::{ethereum::EthBlockHeader, evm::block_header::EvmBlockHeader};

/// [FileProvider] for Ethereum.
pub type EthFileProvider = FileProvider<EthBlockHeader>;

/// A provider returning responses cached in a file.
/// It panics if queries are not found in the cache.
pub type FileProvider<H> = CachedProvider<NullProvider<H>>;

impl<H> FileProvider<H>
where
    H: EvmBlockHeader + Clone + Serialize + DeserializeOwned,
{
    /// Creates a new [FileProvider] loading the given file.
    pub fn from_file(file_path: &PathBuf) -> anyhow::Result<Self> {
        let cache = JsonCache::load(file_path)?;
        Ok(Self {
            inner: NullProvider(PhantomData),
            cache: RefCell::new(cache),
        })
    }
}

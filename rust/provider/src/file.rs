use std::{marker::PhantomData, path::PathBuf, sync::RwLock};

use anyhow::Result;

use super::{
    cache::{json::JsonCache, CachedProvider},
    null::NullProvider,
};

/// A provider returning responses cached in a file.
/// It panics if queries are not found in the cache.
pub type FileProvider = CachedProvider;

impl FileProvider {
    /// Creates a new [FileProvider] loading the given file.
    pub fn from_file(file_path: &PathBuf) -> Result<Self> {
        let cache = JsonCache::load(file_path)?;
        Ok(Self {
            inner: Box::new(NullProvider(PhantomData)),
            cache: RwLock::new(cache),
        })
    }
}

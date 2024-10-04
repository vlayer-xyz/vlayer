use std::{marker::PhantomData, path::PathBuf, sync::RwLock};

use super::{
    cache::{json::JsonCache, CachedProvider},
    null::NullProvider,
};

/// A provider returning responses cached in a file.
/// It panics if queries are not found in the cache.
pub type FileProvider = CachedProvider<NullProvider>;

impl FileProvider {
    /// Creates a new [FileProvider] loading the given file.
    pub fn from_file(file_path: &PathBuf) -> anyhow::Result<Self> {
        let cache = JsonCache::load(file_path)?;
        Ok(Self {
            inner: NullProvider(PhantomData),
            cache: RwLock::new(cache),
        })
    }
}

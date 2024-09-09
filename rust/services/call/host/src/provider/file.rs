use super::{cache::CachedProvider, null::NullProvider};
use crate::provider::cache::json::JsonCache;
use std::{cell::RefCell, marker::PhantomData, path::PathBuf};

/// A provider returning responses cached in a file.
/// It panics if queries are not found in the cache.
pub(crate) type FileProvider = CachedProvider<NullProvider>;

impl FileProvider {
    /// Creates a new [FileProvider] loading the given file.
    pub(crate) fn from_file(file_path: &PathBuf) -> anyhow::Result<Self> {
        let cache = JsonCache::load(file_path)?;
        Ok(Self {
            inner: NullProvider(PhantomData),
            cache: RefCell::new(cache),
        })
    }
}

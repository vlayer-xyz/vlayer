use std::{hash::Hash, sync::Arc};

pub trait InteriorMutabilityCache<K, V: ?Sized>
where
    K: Hash + Eq,
{
    fn get(&self, key: &K) -> Option<Arc<V>>;
    fn try_get_or_insert<F, RV, E>(&self, key: K, f: F) -> Result<Arc<V>, E>
    where
        RV: Into<Arc<V>>,
        F: FnOnce() -> Result<RV, E>;
}

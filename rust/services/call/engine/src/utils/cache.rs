use std::{
    collections::{HashMap, hash_map::Entry},
    hash::Hash,
    sync::{Arc, RwLock},
};

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

impl<K, V: ?Sized> InteriorMutabilityCache<K, V> for RwLock<HashMap<K, Arc<V>>>
where
    K: Hash + Eq,
{
    fn get(&self, key: &K) -> Option<Arc<V>> {
        self.read().expect("poisoned lock").get(key).map(Arc::clone)
    }

    fn try_get_or_insert<F, RV, E>(&self, key: K, f: F) -> Result<Arc<V>, E>
    where
        F: FnOnce() -> Result<RV, E>,
        RV: Into<Arc<V>>,
    {
        let mut cache = self.write().expect("poisoned lock");
        let value = match cache.entry(key) {
            Entry::Occupied(value) => Ok(value.into_mut()),
            Entry::Vacant(entry) => {
                let value = f()?.into();
                Ok(entry.insert(value))
            }
        }?;
        Ok(Arc::clone(value))
    }
}

#[cfg(test)]
mod interior_mutability_cache {
    use std::{
        collections::HashMap,
        sync::{Arc, RwLock},
    };

    use anyhow::{anyhow, bail};

    use super::InteriorMutabilityCache;

    #[test]
    fn found() -> anyhow::Result<()> {
        let cache = RwLock::new(HashMap::from([("key", Arc::new(42))]));

        let value = cache
            .try_get_or_insert::<_, i32, anyhow::Error>("key", || bail!("should not be called"))?;
        assert_eq!(*value, 42);
        Ok(())
    }

    #[test]
    fn created() -> anyhow::Result<()> {
        let cache = RwLock::new(HashMap::new());
        let value = cache
            .try_get_or_insert::<_, i32, anyhow::Error>("key", || Ok::<_, anyhow::Error>(42))?;
        assert_eq!(*value, 42);
        assert_eq!(**cache.read().expect("poisoned lock").get("key").unwrap(), 42);
        Ok(())
    }

    #[test]
    fn failed() -> anyhow::Result<()> {
        let cache = RwLock::new(HashMap::<_, Arc<()>, _>::new());
        let error = cache
            .try_get_or_insert::<_, (), anyhow::Error>("key", || bail!("error"))
            .unwrap_err();
        assert_eq!(error.to_string(), "error");
        assert_eq!(cache.read().expect("poisoned lock").get("key"), None);
        Ok(())
    }

    #[test]
    fn idempotence() -> anyhow::Result<()> {
        let cache = RwLock::new(HashMap::new());
        let call_count = &mut 0;
        let mut return_once = || {
            *call_count += 1;
            if *call_count == 2 {
                Err(anyhow!("error"))
            } else {
                Ok(42)
            }
        };
        cache.try_get_or_insert("key", &mut return_once)?;
        cache.try_get_or_insert("key", &mut return_once)?;
        assert_eq!(**cache.read().expect("poisoned lock").get("key").unwrap(), 42);
        Ok(())
    }
}

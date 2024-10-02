use std::{
    cell::RefCell,
    collections::{hash_map::Entry, HashMap},
    hash::Hash,
    rc::Rc,
};

pub trait InteriorMutabilityCache<K, V>
where
    K: Hash + Eq,
{
    fn get(&self, key: &K) -> Option<Rc<V>>;
    fn try_get_or_insert<F, E>(&self, key: K, f: F) -> Result<Rc<V>, E>
    where
        F: FnOnce() -> Result<V, E>;
}

impl<K, V> InteriorMutabilityCache<K, V> for RefCell<HashMap<K, Rc<V>>>
where
    K: Hash + Eq,
{
    fn get(&self, key: &K) -> Option<Rc<V>> {
        self.borrow().get(key).map(Rc::clone)
    }

    fn try_get_or_insert<F, E>(&self, key: K, f: F) -> Result<Rc<V>, E>
    where
        F: FnOnce() -> Result<V, E>,
    {
        let mut cache = self.borrow_mut();
        let value = match cache.entry(key) {
            Entry::Occupied(value) => Ok(value.into_mut()),
            Entry::Vacant(entry) => {
                let value = Rc::new(f()?);
                Ok(entry.insert(value))
            }
        }?;
        Ok(Rc::clone(value))
    }
}

#[cfg(test)]
mod interior_mutability_cache {
    use std::{cell::RefCell, collections::HashMap, rc::Rc};

    use anyhow::{anyhow, bail};

    use super::InteriorMutabilityCache;

    #[test]
    fn found() -> anyhow::Result<()> {
        let cache = RefCell::new(HashMap::from([("key", Rc::new(42))]));

        let value = cache.try_get_or_insert("key", || bail!("should not be called"))?;
        assert_eq!(*value, 42);
        Ok(())
    }

    #[test]
    fn created() -> anyhow::Result<()> {
        let cache = RefCell::new(HashMap::new());
        let value = cache.try_get_or_insert("key", || Ok::<_, anyhow::Error>(42))?;
        assert_eq!(*value, 42);
        assert_eq!(**cache.borrow().get("key").unwrap(), 42);
        Ok(())
    }

    #[test]
    fn failed() -> anyhow::Result<()> {
        let cache = RefCell::new(HashMap::<_, Rc<()>, _>::new());
        let error = cache
            .try_get_or_insert("key", || bail!("error"))
            .unwrap_err();
        assert_eq!(error.to_string(), "error");
        assert_eq!(cache.borrow().get("key"), None);
        Ok(())
    }

    #[test]
    fn idempotence() -> anyhow::Result<()> {
        let cache = RefCell::new(HashMap::new());
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
        assert_eq!(**cache.borrow().get("key").unwrap(), 42);
        Ok(())
    }
}

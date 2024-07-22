use std::cell::RefCell;
use std::collections::hash_map::Entry;
use std::collections::HashMap;
use std::hash::Hash;
use std::rc::Rc;

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
    use super::InteriorMutabilityCache;
    use anyhow::anyhow;
    use std::{cell::RefCell, collections::HashMap, rc::Rc};

    #[test]
    fn found() {
        let key = "key";
        let value = Rc::new(42);
        let cache = RefCell::new(HashMap::from([(key, Rc::clone(&value))]));

        let result = cache.try_get_or_insert(key, || Err("should not be called"));
        assert_eq!(result, Ok(value));
    }

    #[test]
    fn created() {
        let cache = RefCell::new(HashMap::new());
        let key = "key";
        let value = Rc::new(42);
        let result = cache.try_get_or_insert(key, || Ok::<_, ()>(42));
        assert_eq!(result, Ok(Rc::clone(&value)));
        assert_eq!(cache.borrow().get(key), Some(&value));
    }

    #[test]
    fn failed() {
        let cache = RefCell::new(HashMap::<_, Rc<()>, _>::new());
        let key = "key";
        let error = "error";
        let result = cache.try_get_or_insert(key, || Err(error));
        assert_eq!(result, Err("error"));
        assert_eq!(cache.borrow().get(key), None);
    }

    #[test]
    fn idempotence() -> anyhow::Result<()> {
        let cache = RefCell::new(HashMap::new());
        let key = "key";
        let value = Rc::new(42);
        let call_count = &mut 0;
        let mut return_once = || {
            *call_count += 1;
            if *call_count == 2 {
                Err(anyhow!("error"))
            } else {
                Ok(42)
            }
        };
        cache.try_get_or_insert(key, &mut return_once)?;
        cache.try_get_or_insert(key, &mut return_once)?;
        assert_eq!(cache.borrow().get(key), Some(&value));
        Ok(())
    }
}

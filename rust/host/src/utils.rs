use std::collections::{hash_map::Entry, HashMap};
use std::hash::Hash;

pub trait TryGetOrInsert<K, V>
where
    K: Hash + Eq,
{
    fn try_get_or_insert<F, E>(&mut self, key: K, f: F) -> Result<&mut V, E>
    where
        F: FnOnce() -> Result<V, E>;
}

impl<K, V> TryGetOrInsert<K, V> for HashMap<K, V>
where
    K: Hash + Eq,
{
    fn try_get_or_insert<F, E>(&mut self, key: K, f: F) -> Result<&mut V, E>
    where
        F: FnOnce() -> Result<V, E>,
    {
        match self.entry(key) {
            Entry::Occupied(value) => Ok(value.into_mut()),
            Entry::Vacant(entry) => {
                let value = f()?;
                Ok(entry.insert(value))
            }
        }
    }
}

#[cfg(test)]
mod try_get_or_insert {
    use super::TryGetOrInsert;
    use anyhow::anyhow;
    use std::collections::HashMap;

    #[test]
    fn found() {
        let key = "key";
        let value = 42;
        let mut map = HashMap::from([(key, value)]);

        let result = map.try_get_or_insert(key, || Err("should not be called"));
        assert_eq!(result, Ok(&mut 42));
    }

    #[test]
    fn created() {
        let mut map = HashMap::new();
        let key = "key";
        let value = 42;
        let result = map.try_get_or_insert(key, || Ok::<_, ()>(value));
        assert_eq!(result, Ok(&mut 42));
        assert_eq!(map.get(key), Some(&42));
    }

    #[test]
    fn failed() {
        let mut map = HashMap::<_, (), _>::new();
        let key = "key";
        let error = "error";
        let result = map.try_get_or_insert(key, || Err(error));
        assert_eq!(result, Err("error"));
        assert_eq!(map.get(key), None);
    }

    #[test]
    fn idempotence() -> anyhow::Result<()> {
        let mut map = HashMap::new();
        let key = "key";
        let value = 42;
        let call_count = &mut 0;
        let mut return_once = || {
            *call_count += 1;
            if *call_count == 2 {
                Err(anyhow!("error"))
            } else {
                Ok(value)
            }
        };
        map.try_get_or_insert(key, &mut return_once)?;
        map.try_get_or_insert(key, &mut return_once)?;
        assert_eq!(map.get(key), Some(&42));
        Ok(())
    }
}

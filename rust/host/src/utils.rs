use std::collections::{hash_map::Entry, HashMap};
use std::hash::Hash;

pub fn get_mut_or_insert_with_result<K, V, F, E>(
    map: &mut HashMap<K, V>,
    key: K,
    f: F,
) -> Result<&mut V, E>
where
    K: Hash + Eq,
    F: FnOnce() -> Result<V, E>,
{
    match map.entry(key) {
        Entry::Occupied(value) => Ok(value.into_mut()),
        Entry::Vacant(entry) => {
            let value = f()?;
            Ok(entry.insert(value))
        }
    }
}

#[cfg(test)]
mod get_mut_or_insert_with_result {
    use anyhow::anyhow;

    use super::*;

    #[test]
    fn found() {
        let key = "key";
        let mut value = 42;
        let mut map = HashMap::from([(key, value)]);

        let result = get_mut_or_insert_with_result(&mut map, key, || Err("should not be called"));
        assert_eq!(result, Ok(&mut value));
    }

    #[test]
    fn created() {
        let mut map = HashMap::new();
        let key = "key";
        let mut value = 42;
        let result = get_mut_or_insert_with_result(&mut map, key, || Ok::<_, ()>(value));
        assert_eq!(result, Ok(&mut value));
        assert_eq!(map.get(key), Some(&value));
    }

    #[test]
    fn failed() {
        let mut map = HashMap::<_, (), _>::new();
        let key = "key";
        let error = "error";
        let result = get_mut_or_insert_with_result(&mut map, key, || Err(error));
        assert_eq!(result, Err(error));
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
        get_mut_or_insert_with_result(&mut map, key, &mut return_once)?;
        get_mut_or_insert_with_result(&mut map, key, &mut return_once)?;
        assert_eq!(map.get(key), Some(&value));
        Ok(())
    }
}

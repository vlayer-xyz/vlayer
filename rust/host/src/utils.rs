use std::collections::{hash_map::Entry, HashMap};

pub fn get_or_insert_with_result<K, V, F, E>(map: &mut HashMap<K, V>, key: K, f: F) -> Result<V, E>
where
    K: std::hash::Hash + Eq,
    F: FnOnce() -> Result<V, E>,
    V: Clone,
{
    match map.entry(key) {
        Entry::Occupied(value) => Ok(value.get().clone()),
        Entry::Vacant(entry) => {
            let value = f()?;
            entry.insert(value.clone());
            Ok(value)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_or_insert_with_result() {
        let mut map = HashMap::new();
        let key = "key";
        let value = 42;
        let result = get_or_insert_with_result(&mut map, key, || Ok::<_, ()>(value));
        assert_eq!(result, Ok(value));
        assert_eq!(map.get(key), Some(&value));
    }

    #[test]
    fn test_get_or_insert_with_result_error() {
        let mut map = HashMap::<_, (), _>::new();
        let key = "key";
        let error = "error";
        let result = get_or_insert_with_result(&mut map, key, || Err(error));
        assert_eq!(result, Err(error));
        assert_eq!(map.get(key), None);
    }
}

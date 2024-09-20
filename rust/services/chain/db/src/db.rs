use std::{collections::HashMap, sync::RwLock};
use thiserror::Error;

#[allow(unused)]
pub trait KeyValueDB {
    fn insert(&self, key: impl AsRef<[u8]>, value: impl AsRef<[u8]>)
        -> Result<(), KeyValueDBError>;
    fn get(&self, key: impl AsRef<[u8]>) -> Option<Box<[u8]>>;
}

#[derive(Error, Debug, PartialEq)]
pub enum KeyValueDBError {
    #[error("duplicate key")]
    DuplicateKey,
}

type KeyValueMap = HashMap<Box<[u8]>, Box<[u8]>>;

pub struct InMemoryKeyValueDB {
    store: RwLock<KeyValueMap>,
}

impl InMemoryKeyValueDB {
    #[allow(unused)]
    pub fn new() -> Self {
        InMemoryKeyValueDB {
            store: RwLock::new(HashMap::new()),
        }
    }
}

impl KeyValueDB for InMemoryKeyValueDB {
    fn insert(
        &self,
        key: impl AsRef<[u8]>,
        value: impl AsRef<[u8]>,
    ) -> Result<(), KeyValueDBError> {
        let key_ref = key.as_ref();
        let mut store = self.store.write().unwrap();
        if store.contains_key(key_ref) {
            return Err(KeyValueDBError::DuplicateKey);
        }
        store.insert(key_ref.into(), value.as_ref().into());
        Ok(())
    }

    fn get(&self, key: impl AsRef<[u8]>) -> Option<Box<[u8]>> {
        let store = self.store.read().unwrap();
        store.get(key.as_ref()).cloned()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn insert_and_get() -> anyhow::Result<()> {
        let db = InMemoryKeyValueDB::new();

        db.insert([0], [42])?;

        assert_eq!(*db.get([0]).unwrap(), [42]);

        Ok(())
    }

    #[test]
    fn insert_duplicate_key() {
        let db = InMemoryKeyValueDB::new();

        db.insert([0], [42]).unwrap();

        assert_eq!(db.insert([0], [42]), Err(KeyValueDBError::DuplicateKey));
    }

    #[test]
    fn get_nonexistent_key() -> anyhow::Result<()> {
        let db = InMemoryKeyValueDB::new();

        assert_eq!(db.get([0]), None);

        Ok(())
    }
}

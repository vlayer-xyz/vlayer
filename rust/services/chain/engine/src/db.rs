use std::collections::HashMap;
use thiserror::Error;

#[allow(unused)]
pub trait KeyValueDB {
    fn insert(
        &mut self,
        key: impl AsRef<[u8]>,
        value: impl AsRef<[u8]>,
    ) -> Result<(), KeyValueDBError>;
    fn get(&self, key: impl AsRef<[u8]>) -> Option<Box<[u8]>>;
}

#[derive(Error, Debug, PartialEq)]
pub enum KeyValueDBError {
    #[error("duplicate key")]
    DuplicateKey,
}

pub struct InMemoryKeyValueDB {
    store: HashMap<Box<[u8]>, Box<[u8]>>,
}

impl InMemoryKeyValueDB {
    #[allow(unused)]
    pub fn new() -> Self {
        InMemoryKeyValueDB {
            store: HashMap::new(),
        }
    }
}

impl KeyValueDB for InMemoryKeyValueDB {
    fn insert(
        &mut self,
        key: impl AsRef<[u8]>,
        value: impl AsRef<[u8]>,
    ) -> Result<(), KeyValueDBError> {
        let key = key.as_ref();
        if self.store.contains_key(key) {
            return Err(KeyValueDBError::DuplicateKey);
        }
        self.store.insert(key.into(), value.as_ref().into());
        Ok(())
    }

    fn get(&self, key: impl AsRef<[u8]>) -> Option<Box<[u8]>> {
        self.store.get(key.as_ref()).cloned()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn insert_and_get() -> anyhow::Result<()> {
        let mut db = InMemoryKeyValueDB::new();

        db.insert([0], [42])?;

        assert_eq!(*db.get([0]).unwrap(), [42]);

        Ok(())
    }

    #[test]
    fn insert_duplicate_key() {
        let mut db = InMemoryKeyValueDB::new();

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

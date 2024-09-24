use std::collections::HashMap;

use crate::{Database, DatabaseError, RoTransaction, RwTransaction};

type KeyValueMap = HashMap<Box<[u8]>, Box<[u8]>>;

#[allow(unused)]
pub struct InMemoryDatabase {
    store: KeyValueMap,
}

impl Database for InMemoryDatabase {
    type RoTx = InMemoryRoTransaction;
    type RwTx = InMemoryRwTransaction;

    fn begin_ro(&self) -> InMemoryRoTransaction {
        InMemoryRoTransaction {
            store: self.store.clone(),
        }
    }

    fn begin_rw(&mut self) -> InMemoryRwTransaction {
        InMemoryRwTransaction {
            store: self.store.clone(),
        }
    }
}

impl InMemoryDatabase {
    #[allow(unused)]
    fn new() -> Self {
        InMemoryDatabase {
            store: HashMap::new(),
        }
    }
}

pub struct InMemoryRoTransaction {
    store: KeyValueMap,
}

impl RoTransaction for InMemoryRoTransaction {
    fn get(&self, table: impl AsRef<str>, key: impl AsRef<[u8]>) -> Option<Box<[u8]>> {
        let prefixed_key = add_table_prefix(table, key);
        self.store.get(prefixed_key.as_slice()).cloned()
    }
}

pub struct InMemoryRwTransaction {
    store: KeyValueMap,
}

impl RoTransaction for InMemoryRwTransaction {
    fn get(&self, table: impl AsRef<str>, key: impl AsRef<[u8]>) -> Option<Box<[u8]>> {
        let prefixed_key = add_table_prefix(table, key);
        self.store.get(prefixed_key.as_slice()).cloned()
    }
}

fn add_table_prefix(table: impl AsRef<str>, key: impl AsRef<[u8]>) -> Vec<u8> {
    [table.as_ref().as_bytes(), key.as_ref()].concat()
}

impl RwTransaction for InMemoryRwTransaction {
    fn insert(
        &mut self,
        table: impl AsRef<str>,
        key: impl AsRef<[u8]>,
        value: impl AsRef<[u8]>,
    ) -> Result<(), DatabaseError> {
        let prefixed_key = add_table_prefix(table, key);
        if self.store.contains_key(prefixed_key.as_slice()) {
            return Err(DatabaseError::DuplicateKey);
        }
        self.store
            .insert(prefixed_key.into(), value.as_ref().into());
        Ok(())
    }

    fn delete(
        &mut self,
        table: impl AsRef<str>,
        key: impl AsRef<[u8]>,
    ) -> Result<bool, DatabaseError> {
        let prefixed_key = add_table_prefix(table, key);
        Ok(self.store.remove(prefixed_key.as_slice()).is_some())
    }

    fn commit(self) -> Result<(), DatabaseError> {
        Ok(())
    }
}

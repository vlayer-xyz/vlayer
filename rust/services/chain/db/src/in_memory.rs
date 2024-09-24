use std::collections::HashMap;

use crate::{Database, DatabaseError, RoTransaction, RwTransaction};

type KeyValueMap = HashMap<Box<[u8]>, Box<[u8]>>;

#[allow(unused)]
pub struct InMemoryDatabase {
    store: KeyValueMap,
}

pub struct InMemoryRoTransaction<'a> {
    store: &'a KeyValueMap,
}

pub struct InMemoryRwTransaction<'a> {
    store: &'a mut KeyValueMap,
}

impl<'a> Database<'a> for InMemoryDatabase {
    type RoTx = InMemoryRoTransaction<'a>;
    type RwTx = InMemoryRwTransaction<'a>;

    fn begin_ro(&self) -> InMemoryRoTransaction {
        InMemoryRoTransaction { store: &self.store }
    }

    fn begin_rw(&mut self) -> InMemoryRwTransaction {
        InMemoryRwTransaction {
            store: &mut self.store,
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

impl<'a> RoTransaction for InMemoryRoTransaction<'a> {
    fn get(&self, table: impl AsRef<str>, key: impl AsRef<[u8]>) -> Option<Box<[u8]>> {
        let prefixed_key = add_table_prefix(table, key);
        self.store.get(prefixed_key.as_slice()).cloned()
    }
}

impl<'a> RoTransaction for InMemoryRwTransaction<'a> {
    fn get(&self, table: impl AsRef<str>, key: impl AsRef<[u8]>) -> Option<Box<[u8]>> {
        let prefixed_key = add_table_prefix(table, key);
        self.store.get(prefixed_key.as_slice()).cloned()
    }
}

fn add_table_prefix(table: impl AsRef<str>, key: impl AsRef<[u8]>) -> Vec<u8> {
    [table.as_ref().as_bytes(), key.as_ref()].concat()
}

impl<'a> RwTransaction for InMemoryRwTransaction<'a> {
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
    ) -> Result<(), DatabaseError> {
        let prefixed_key = add_table_prefix(table, key);
        if self.store.remove(prefixed_key.as_slice()).is_some() {
            Ok(())
        } else {
            Err(DatabaseError::NonExistingKey)
        }
    }

    fn commit(self) -> Result<(), DatabaseError> {
        Ok(())
    }
}

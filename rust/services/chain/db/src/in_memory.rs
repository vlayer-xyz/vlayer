use std::collections::HashMap;

use crate::{Database, DatabaseError, ReadTx, ReadWriteTx, WriteTx};

type KeyValueMap = HashMap<Box<[u8]>, Box<[u8]>>;

#[allow(unused)]
pub struct InMemoryDatabase {
    store: KeyValueMap,
}

pub struct InMemoryReadTx<'a> {
    store: &'a KeyValueMap,
}

pub struct InMemoryReadWriteTx<'a> {
    store: &'a mut KeyValueMap,
}

impl<'a> Database<'a> for InMemoryDatabase {
    type ReadTx = InMemoryReadTx<'a>;
    type ReadWriteTx = InMemoryReadWriteTx<'a>;

    fn begin_ro(&'a self) -> Result<Self::ReadTx, DatabaseError> {
        Ok(InMemoryReadTx { store: &self.store })
    }

    fn begin_rw(&'a mut self) -> Result<Self::ReadWriteTx, DatabaseError> {
        Ok(InMemoryReadWriteTx {
            store: &mut self.store,
        })
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

impl<'a> ReadTx for InMemoryReadTx<'a> {
    fn get(
        &self,
        table: impl AsRef<str>,
        key: impl AsRef<[u8]>,
    ) -> Result<Option<Box<[u8]>>, DatabaseError> {
        let prefixed_key = add_table_prefix(table, key);
        Ok(self.store.get(prefixed_key.as_slice()).cloned())
    }
}

impl<'a> ReadTx for InMemoryReadWriteTx<'a> {
    fn get(
        &self,
        table: impl AsRef<str>,
        key: impl AsRef<[u8]>,
    ) -> Result<Option<Box<[u8]>>, DatabaseError> {
        let prefixed_key = add_table_prefix(table, key);
        Ok(self.store.get(prefixed_key.as_slice()).cloned())
    }
}

fn add_table_prefix(table: impl AsRef<str>, key: impl AsRef<[u8]>) -> Vec<u8> {
    [table.as_ref().as_bytes(), key.as_ref()].concat()
}

impl<'a> WriteTx for InMemoryReadWriteTx<'a> {
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

impl<'a> ReadWriteTx for InMemoryReadWriteTx<'a> {}

use std::collections::HashMap;

use crate::{Database, DbError, DbResult, ReadTx, ReadWriteTx, WriteTx};

type KeyValueMap = HashMap<Box<[u8]>, Box<[u8]>>;

#[derive(Default)]
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
    fn begin_ro(&'a self) -> DbResult<Box<dyn ReadTx + 'a>> {
        Ok(Box::new(InMemoryReadTx { store: &self.store }))
    }

    fn begin_rw(&'a mut self) -> DbResult<Box<dyn ReadWriteTx + 'a>> {
        Ok(Box::new(InMemoryReadWriteTx {
            store: &mut self.store,
        }))
    }
}

impl InMemoryDatabase {
    pub fn new() -> Self {
        InMemoryDatabase {
            store: HashMap::new(),
        }
    }
}

impl ReadTx for InMemoryReadTx<'_> {
    fn get(&self, table: &str, key: &[u8]) -> DbResult<Option<Box<[u8]>>> {
        let prefixed_key = add_table_prefix(table, key);
        Ok(self.store.get(prefixed_key.as_slice()).cloned())
    }
}

impl ReadTx for InMemoryReadWriteTx<'_> {
    fn get(&self, table: &str, key: &[u8]) -> DbResult<Option<Box<[u8]>>> {
        let prefixed_key = add_table_prefix(table, key);
        Ok(self.store.get(prefixed_key.as_slice()).cloned())
    }
}

fn add_table_prefix(table: impl AsRef<str>, key: impl AsRef<[u8]>) -> Vec<u8> {
    [table.as_ref().as_bytes(), key.as_ref()].concat()
}

impl WriteTx for InMemoryReadWriteTx<'_> {
    fn create_table(&mut self, table: &str) -> DbResult<()> {
        Ok(())
    }

    fn insert(&mut self, table: &str, key: &[u8], value: &[u8]) -> DbResult<()> {
        let prefixed_key = add_table_prefix(table, key);
        if self.store.contains_key(prefixed_key.as_slice()) {
            return Err(DbError::duplicate_key(table, key));
        }
        self.store
            .insert(prefixed_key.into(), value.as_ref().into());
        Ok(())
    }

    fn upsert(&mut self, table: &str, key: &[u8], value: &[u8]) -> DbResult<()> {
        let prefixed_key = add_table_prefix(table, key);
        self.store
            .insert(prefixed_key.into(), value.as_ref().into());
        Ok(())
    }

    fn delete(&mut self, table: &str, key: &[u8]) -> DbResult<()> {
        let prefixed_key = add_table_prefix(table, key);
        if self.store.remove(prefixed_key.as_slice()).is_some() {
            Ok(())
        } else {
            Err(DbError::non_existing_key(table, key))
        }
    }

    fn commit(self: Box<Self>) -> DbResult<()> {
        Ok(())
    }
}

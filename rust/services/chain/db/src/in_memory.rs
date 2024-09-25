use std::collections::HashMap;

use crate::{Database, DbError, DbResult, ReadTx, ReadWriteTx, WriteTx};

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

    fn begin_ro(&'a self) -> DbResult<Self::ReadTx> {
        Ok(InMemoryReadTx { store: &self.store })
    }

    fn begin_rw(&'a mut self) -> DbResult<Self::ReadWriteTx> {
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
    fn get(&self, table: impl AsRef<str>, key: impl AsRef<[u8]>) -> DbResult<Option<Box<[u8]>>> {
        let prefixed_key = add_table_prefix(table, key);
        Ok(self.store.get(prefixed_key.as_slice()).cloned())
    }
}

impl<'a> ReadTx for InMemoryReadWriteTx<'a> {
    fn get(&self, table: impl AsRef<str>, key: impl AsRef<[u8]>) -> DbResult<Option<Box<[u8]>>> {
        let prefixed_key = add_table_prefix(table, key);
        Ok(self.store.get(prefixed_key.as_slice()).cloned())
    }
}

fn add_table_prefix(table: impl AsRef<str>, key: impl AsRef<[u8]>) -> Vec<u8> {
    [table.as_ref().as_bytes(), key.as_ref()].concat()
}

impl<'a> WriteTx for InMemoryReadWriteTx<'a> {
    fn create_table(&mut self, table: impl AsRef<str>) -> DbResult<()> {
        Ok(())
    }

    fn insert(
        &mut self,
        table: impl AsRef<str>,
        key: impl AsRef<[u8]>,
        value: impl AsRef<[u8]>,
    ) -> DbResult<()> {
        let prefixed_key = add_table_prefix(table, key);
        if self.store.contains_key(prefixed_key.as_slice()) {
            return Err(DbError::DuplicateKey);
        }
        self.store
            .insert(prefixed_key.into(), value.as_ref().into());
        Ok(())
    }

    fn delete(&mut self, table: impl AsRef<str>, key: impl AsRef<[u8]>) -> DbResult<()> {
        let prefixed_key = add_table_prefix(table, key);
        if self.store.remove(prefixed_key.as_slice()).is_some() {
            Ok(())
        } else {
            Err(DbError::NonExistingKey)
        }
    }

    fn commit(self) -> DbResult<()> {
        Ok(())
    }
}

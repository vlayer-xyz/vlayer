use std::{
    collections::HashMap,
    sync::{RwLock, RwLockReadGuard, RwLockWriteGuard},
};
use thiserror::Error;

#[allow(unused)]
pub trait KeyValueDB {
    type RoTx: ReadDBTx;
    type RwTx: RwDBTx;

    fn begin_ro(&self) -> Self::RoTx;
    fn begin_rw(&mut self) -> Self::RwTx;
}

#[allow(unused)]
pub trait ReadDBTx {
    fn get(&self, table: impl AsRef<str>, key: &[u8]) -> Option<Box<[u8]>>;
}

#[allow(unused)]
pub trait RwDBTx: ReadDBTx {
    fn insert(
        &mut self,
        table: impl AsRef<str>,
        key: &[u8],
        value: &[u8],
    ) -> Result<(), KeyValueDBError>;
    fn delete(&mut self, table: impl AsRef<str>, key: &[u8]) -> Result<bool, KeyValueDBError>;
    fn commit(self) -> Result<(), KeyValueDBError>;
}

#[derive(Error, Debug, PartialEq)]
pub enum KeyValueDBError {
    #[error("duplicate key")]
    DuplicateKey,
    #[error("{0}")]
    #[allow(unused)]
    Custom(String), //todo: Implement associated error type for KeyValueDB (?)
}

type KeyValueMap = HashMap<Box<[u8]>, Box<[u8]>>;

#[allow(unused)]
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

pub struct InMemoryReadOnlyTx<'a> {
    store: RwLockReadGuard<'a, KeyValueMap>,
}

impl<'a> ReadDBTx for InMemoryReadOnlyTx<'a> {
    fn get(&self, table: impl AsRef<str>, key: &[u8]) -> Option<Box<[u8]>> {
        let prefixed_key = add_table_prefix(table, key);
        self.store.get(prefixed_key.as_slice()).cloned()
    }
}

pub struct InMemoryReadWriteTx<'a> {
    store: RwLockWriteGuard<'a, KeyValueMap>,
}

impl<'a> ReadDBTx for InMemoryReadWriteTx<'a> {
    fn get(&self, table: impl AsRef<str>, key: &[u8]) -> Option<Box<[u8]>> {
        let prefixed_key = add_table_prefix(table, key);
        self.store.get(prefixed_key.as_slice()).cloned()
    }
}

fn add_table_prefix(table: impl AsRef<str>, key: &[u8]) -> Vec<u8> {
    let mut prefixed_key = Vec::with_capacity(table.as_ref().len() + key.len());
    prefixed_key.extend_from_slice(table.as_ref().as_bytes());
    prefixed_key.extend_from_slice(key);
    prefixed_key
}

impl<'a> RwDBTx for InMemoryReadWriteTx<'a> {
    fn insert(
        &mut self,
        table: impl AsRef<str>,
        key: &[u8],
        value: &[u8],
    ) -> Result<(), KeyValueDBError> {
        let prefixed_key = add_table_prefix(table, key);
        if self.store.contains_key(prefixed_key.as_slice()) {
            return Err(KeyValueDBError::DuplicateKey);
        }
        self.store.insert(prefixed_key.into(), value.into());
        Ok(())
    }

    fn delete(&mut self, table: impl AsRef<str>, key: &[u8]) -> Result<bool, KeyValueDBError> {
        let prefixed_key = add_table_prefix(table, key);
        Ok(self.store.remove(prefixed_key.as_slice()).is_some())
    }

    fn commit(self) -> Result<(), KeyValueDBError> {
        Ok(())
    }
}

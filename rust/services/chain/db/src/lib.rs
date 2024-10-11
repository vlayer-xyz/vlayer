#![allow(unused)]

use static_assertions::assert_obj_safe;
use thiserror::Error;

mod chain_db;
mod in_memory;
mod mdbx;

pub use chain_db::{ChainDb, ChainDbError, ChainDbResult, ChainInfo, ChainTrie, ChainUpdate};
pub use in_memory::InMemoryDatabase;
pub use mdbx::Mdbx;

pub trait Database<'a> {
    type ReadTx: ReadTx + 'a;
    type ReadWriteTx: ReadWriteTx + 'a;

    fn begin_ro(&'a self) -> DbResult<Self::ReadTx>;
    fn begin_rw(&'a mut self) -> DbResult<Self::ReadWriteTx>;

    fn with_ro_tx<T, F>(&'a self, f: F) -> DbResult<T>
    where
        F: FnOnce(&Self::ReadTx) -> DbResult<T> + Sized,
    {
        let tx = self.begin_ro()?;
        f(&tx)
    }

    fn with_rw_tx<T, F>(&'a mut self, f: F) -> DbResult<T>
    where
        F: FnOnce(&mut Self::ReadWriteTx) -> DbResult<T>,
    {
        let mut tx = self.begin_rw()?;
        let res = f(&mut tx)?;
        tx.commit()?;
        Ok(res)
    }
}

pub trait ReadTx {
    fn get(&self, table: &str, key: &[u8]) -> DbResult<Option<Box<[u8]>>>;
}

assert_obj_safe!(ReadTx);

// While nothing in code require a mutable reference in insert and delete, we want to
// discourage the user from sharing a write transaction as this can lead to db data races
pub trait WriteTx {
    fn create_table(&mut self, table: &str) -> DbResult<()>;
    /// Insert `(key, value)` into `table`. Returns `DbError::DuplicateKey` if `key` alredy exists in `table`.
    fn insert(&mut self, table: &str, key: &[u8], value: &[u8]) -> DbResult<()>;
    /// Insert `(key, value)` into `table` or update to value if `key` already exists in `table`.
    fn upsert(&mut self, table: &str, key: &[u8], value: &[u8]) -> DbResult<()>;
    fn delete(&mut self, table: &str, key: &[u8]) -> DbResult<()>;
    fn commit(self) -> DbResult<()>;
}

assert_obj_safe!(WriteTx);

pub trait ReadWriteTx: ReadTx + WriteTx {}

assert_obj_safe!(ReadWriteTx);

impl<T: ReadTx + WriteTx> ReadWriteTx for T {}

#[derive(Error, Debug, PartialEq, Eq, Clone)]
pub enum DbError {
    #[error("Specified key already exists. table='{table}' key='{key:?}'")]
    DuplicateKey { table: Box<str>, key: Box<[u8]> },
    #[error("Specified key doesn't exist. table='{table}' key='{key:?}'")]
    NonExistingKey { table: Box<str>, key: Box<[u8]> },
    #[error("Specified table doesn't exist: {0}")]
    NonExistingTable(Box<str>),
    #[error("{0}")]
    Custom(Box<str>), //todo: Implement associated error type for KeyValueDB (?)
}

impl DbError {
    pub fn duplicate_key(table: impl AsRef<str>, key: impl AsRef<[u8]>) -> Self {
        Self::DuplicateKey {
            table: table.as_ref().into(),
            key: key.as_ref().into(),
        }
    }

    pub fn non_existing_key(table: impl AsRef<str>, key: impl AsRef<[u8]>) -> Self {
        Self::NonExistingKey {
            table: table.as_ref().into(),
            key: key.as_ref().into(),
        }
    }

    pub fn non_existing_table(table: impl AsRef<str>) -> Self {
        Self::NonExistingTable(table.as_ref().into())
    }

    #[allow(clippy::needless_pass_by_value)] // More convenient to map errors
    pub fn custom(err: impl ToString) -> Self {
        Self::Custom(err.to_string().into_boxed_str())
    }
}

pub type DbResult<T> = Result<T, DbError>;

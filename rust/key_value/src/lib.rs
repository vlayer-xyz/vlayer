#![allow(unused)]

use auto_impl::auto_impl;
use static_assertions::assert_obj_safe;
use thiserror::Error;

mod in_memory;
mod mdbx;

pub use in_memory::InMemoryDatabase;
pub use mdbx::Mdbx;

pub trait Database<'a> {
    fn begin_ro(&'a self) -> DbResult<Box<dyn ReadTx + 'a>>;
    fn begin_rw(&'a mut self) -> DbResult<Box<dyn ReadWriteTx + 'a>>;
}

assert_obj_safe!(Database);

#[auto_impl(Box)]
pub trait ReadTx {
    fn get(&self, table: &str, key: &[u8]) -> DbResult<Option<Box<[u8]>>>;
}

assert_obj_safe!(ReadTx);

// While nothing in code require a mutable reference in insert and delete, we want to
// discourage the user from sharing a write transaction as this can lead to db data races
#[auto_impl(Box)]
pub trait WriteTx {
    fn create_table(&mut self, table: &str) -> DbResult<()>;
    /// Insert `(key, value)` into `table`. Returns `DbError::DuplicateKey` if `key` alredy exists in `table`.
    fn insert(&mut self, table: &str, key: &[u8], value: &[u8]) -> DbResult<()>;
    /// Insert `(key, value)` into `table` or update to value if `key` already exists in `table`.
    fn upsert(&mut self, table: &str, key: &[u8], value: &[u8]) -> DbResult<()>;
    fn delete(&mut self, table: &str, key: &[u8]) -> DbResult<()>;
    // Box wrapping is needed for a trait to be object-safe
    fn commit(self: Box<Self>) -> DbResult<()>;
}

assert_obj_safe!(WriteTx);

pub trait ReadWriteTx: ReadTx + WriteTx {}

assert_obj_safe!(ReadWriteTx);

impl<T: ReadTx + WriteTx + ?Sized> ReadWriteTx for T {}

#[derive(Error, Debug, PartialEq, Eq, Clone)]
pub enum DbError {
    #[error("Specified key already exists. table='{table}' key='{key:?}'")]
    DuplicateKey { table: Box<str>, key: Box<[u8]> },
    #[error("Specified key doesn't exist. table='{table}' key='{key:?}'")]
    NonExistingKey { table: Box<str>, key: Box<[u8]> },
    #[error("Specified table doesn't exist: {0}")]
    NonExistingTable(Box<str>),
    #[error("{0}")]
    Custom(Box<str>),
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

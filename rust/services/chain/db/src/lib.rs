#![allow(unused)]

use thiserror::Error;

mod in_memory;

pub trait Database<'a> {
    type ReadTx: ReadTx + 'a;
    type ReadWriteTx: ReadWriteTx + 'a;

    fn begin_ro(&'a self) -> DbResult<Self::ReadTx>;
    fn begin_rw(&'a mut self) -> DbResult<Self::ReadWriteTx>;
}

pub trait ReadTx {
    fn get(&self, table: impl AsRef<str>, key: impl AsRef<[u8]>) -> DbResult<Option<Box<[u8]>>>;
}

// While nothing in code require a mutable reference in insert and delete, we want to
// discourage the user from sharing a write transaction as this can lead to db data races
pub trait WriteTx {
    fn insert(
        &mut self,
        table: impl AsRef<str>,
        key: impl AsRef<[u8]>,
        value: impl AsRef<[u8]>,
    ) -> DbResult<()>;

    fn delete(&mut self, table: impl AsRef<str>, key: impl AsRef<[u8]>) -> DbResult<()>;

    fn commit(self) -> DbResult<()>;
}

pub trait ReadWriteTx: ReadTx + WriteTx {}

impl<T: ReadTx + WriteTx> ReadWriteTx for T {}

#[derive(Error, Debug, PartialEq)]
pub enum DbError {
    #[error("duplicate key")]
    DuplicateKey,
    #[error("non existing key")]
    NonExistingKey,
    #[error("{0}")]
    Custom(String), //todo: Implement associated error type for KeyValueDB (?)
}

pub type DbResult<T> = Result<T, DbError>;

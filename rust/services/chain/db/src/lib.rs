#![allow(unused)]

use thiserror::Error;

mod in_memory;
mod mdbx;

pub trait Database<'a> {
    type ReadTx: ReadTx + 'a;
    type ReadWriteTx: ReadWriteTx + 'a;

    fn begin_ro(&'a self) -> DbResult<Self::ReadTx>;

    fn begin_rw(&'a mut self) -> DbResult<Self::ReadWriteTx>;

    fn with_ro_tx<T, F: FnOnce(&Self::ReadTx) -> DbResult<T>>(&'a self, f: F) -> DbResult<T> {
        let tx = self.begin_ro()?;
        f(&tx)
    }

    fn with_rw_tx<T, F: FnOnce(&mut Self::ReadWriteTx) -> DbResult<T>>(
        &'a mut self,
        f: F,
    ) -> DbResult<T> {
        let mut tx = self.begin_rw()?;
        let res = f(&mut tx)?;
        tx.commit()?;
        Ok(res)
    }
}

pub trait ReadTx {
    fn get(&self, table: impl AsRef<str>, key: impl AsRef<[u8]>) -> DbResult<Option<Box<[u8]>>>;
}

// While nothing in code require a mutable reference in insert and delete, we want to
// discourage the user from sharing a write transaction as this can lead to db data races
pub trait WriteTx {
    fn create_table(&mut self, table: impl AsRef<str>) -> DbResult<()>;

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
    #[error("Specified key already exists")]
    DuplicateKey,
    #[error("Specified key doesn't exist")]
    NonExistingKey,
    #[error("Specified table doesn't exist")]
    NonExistingTable,
    #[error("{0}")]
    Custom(String), //todo: Implement associated error type for KeyValueDB (?)
}

pub type DbResult<T> = Result<T, DbError>;

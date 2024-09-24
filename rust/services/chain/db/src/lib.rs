use thiserror::Error;

mod in_memory;

#[allow(unused)]
pub trait Database<'a> {
    type ReadTx: ReadTx + 'a;
    type ReadWriteTx: ReadWriteTx + 'a;

    fn begin_ro(&'a self) -> Result<Self::ReadTx, DatabaseError>;
    fn begin_rw(&'a mut self) -> Result<Self::ReadWriteTx, DatabaseError>;
}

#[allow(unused)]
pub trait ReadTx {
    fn get(
        &self,
        table: impl AsRef<str>,
        key: impl AsRef<[u8]>,
    ) -> Result<Option<Box<[u8]>>, DatabaseError>;
}

// While nothing in code require a mutable reference in insert and delete, we want to
// discourage the user from sharing a write transaction as this can lead to db data races
#[allow(unused)]
pub trait WriteTx {
    fn insert(
        &mut self,
        table: impl AsRef<str>,
        key: impl AsRef<[u8]>,
        value: impl AsRef<[u8]>,
    ) -> Result<(), DatabaseError>;
    fn delete(
        &mut self,
        table: impl AsRef<str>,
        key: impl AsRef<[u8]>,
    ) -> Result<(), DatabaseError>;
    fn commit(self) -> Result<(), DatabaseError>;
}

pub trait ReadWriteTx: ReadTx + WriteTx {}

#[derive(Error, Debug, PartialEq)]
pub enum DatabaseError {
    #[error("duplicate key")]
    DuplicateKey,
    #[error("{0}")]
    #[allow(unused)]
    Custom(String), //todo: Implement associated error type for KeyValueDB (?)
    #[error("non existing key")]
    #[allow(unused)]
    NonExistingKey,
}

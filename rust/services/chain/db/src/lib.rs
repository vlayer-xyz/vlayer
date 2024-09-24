use thiserror::Error;

mod in_memory;

#[allow(unused)]
pub trait Database {
    type RoTx: RoTransaction;
    type RwTx: RwTransaction;

    fn begin_ro(&self) -> Self::RoTx;
    fn begin_rw(&mut self) -> Self::RwTx;
}

#[allow(unused)]
pub trait RoTransaction {
    fn get(&self, table: impl AsRef<str>, key: impl AsRef<[u8]>) -> Option<Box<[u8]>>;
}

// While nothing in code require a mutable reference in insert and delete, we want to
// discourage the user from sharing a write transaction as this can lead to db data races
#[allow(unused)]
pub trait RwTransaction {
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
    ) -> Result<bool, DatabaseError>;
    fn commit(self) -> Result<(), DatabaseError>;
}

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

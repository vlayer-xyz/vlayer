use std::{f32::consts::E, path::Path};

use libmdbx::{
    DatabaseOptions, Table, TableFlags, Transaction, TransactionKind, WriteFlags, WriteMap, RO, RW,
};

use super::{DbError, DbResult, ReadTx, WriteTx};

pub const MAX_TABLES: u64 = 1024;

pub struct Mdbx {
    db: libmdbx::Database<WriteMap>,
}

impl Mdbx {
    pub fn open(path: impl AsRef<Path>) -> DbResult<Self> {
        let db_opts = DatabaseOptions {
            max_tables: Some(MAX_TABLES),
            ..Default::default()
        };
        let db = libmdbx::Database::open_with_options(path, db_opts).map_err(DbError::custom)?;
        Ok(Self { db })
    }
}

impl<'a> crate::Database<'a> for Mdbx {
    type ReadTx = MdbxTx<'a, RO>;
    type ReadWriteTx = MdbxTx<'a, RW>;

    fn begin_ro(&'a self) -> DbResult<Self::ReadTx> {
        let tx = self.db.begin_ro_txn().map_err(DbError::custom)?;
        Ok(MdbxTx { tx })
    }

    fn begin_rw(&'a mut self) -> DbResult<Self::ReadWriteTx> {
        let tx = self.db.begin_rw_txn().map_err(DbError::custom)?;
        Ok(MdbxTx { tx })
    }
}

pub struct MdbxTx<'a, TK: TransactionKind> {
    tx: Transaction<'a, TK, WriteMap>,
}

impl<'a, TK: TransactionKind> MdbxTx<'a, TK> {
    fn get_table(&'a self, table: impl AsRef<str>) -> DbResult<Table<'a>> {
        match self.tx.open_table(table.as_ref().into()) {
            Ok(table) => Ok(table),
            Err(libmdbx::Error::NotFound) => Err(DbError::non_existing_table(table)),
            Err(err) => Err(DbError::custom(err)),
        }
    }
}

impl<'a, TK: TransactionKind> ReadTx for MdbxTx<'a, TK> {
    fn get(&self, table: impl AsRef<str>, key: impl AsRef<[u8]>) -> DbResult<Option<Box<[u8]>>> {
        let table = self.get_table(table)?;
        Ok(self
            .tx
            .get::<Vec<u8>>(&table, key.as_ref())
            .map_err(DbError::custom)?
            .map(Vec::into_boxed_slice))
    }
}

impl<'a> WriteTx for MdbxTx<'a, RW> {
    fn create_table(&mut self, table: impl AsRef<str>) -> DbResult<()> {
        // `create_table` creates only if the table doesn't exist
        self.tx
            .create_table(table.as_ref().into(), TableFlags::CREATE)
            .map_err(DbError::custom)?;
        Ok(())
    }

    fn insert(
        &mut self,
        table: impl AsRef<str>,
        key: impl AsRef<[u8]>,
        value: impl AsRef<[u8]>,
    ) -> DbResult<()> {
        let mdbx_table = self.get_table(&table)?;
        self.tx
            .put(&mdbx_table, &key, value, WriteFlags::NO_OVERWRITE)
            .map_err(|err| match err {
                libmdbx::Error::KeyExist => DbError::duplicate_key(table, key),
                _ => DbError::custom(err),
            })
    }

    fn upsert(
        &mut self,
        table: impl AsRef<str>,
        key: impl AsRef<[u8]>,
        value: impl AsRef<[u8]>,
    ) -> DbResult<()> {
        let mdbx_table = self.get_table(&table)?;
        self.tx
            .put(&mdbx_table, &key, value, WriteFlags::default())
            .map_err(DbError::custom)
    }

    fn delete(&mut self, table: impl AsRef<str>, key: impl AsRef<[u8]>) -> DbResult<()> {
        let mdbx_table = self.get_table(&table)?;
        self.tx
            .del(&mdbx_table, &key, None)
            .map_err(DbError::custom)?
            .then_some(())
            .ok_or(DbError::non_existing_key(table, key))
    }

    fn commit(self) -> DbResult<()> {
        self.tx.commit().map_err(DbError::custom)?;
        Ok(())
    }
}

#[cfg(test)]
mod tests;

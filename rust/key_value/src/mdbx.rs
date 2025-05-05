use std::{f32::consts::E, io::Read, path::Path};

use libmdbx::{
    DatabaseOptions, RO, RW, ReadWriteOptions, Table, TableFlags, Transaction, TransactionKind,
    WriteFlags, WriteMap,
};

use super::{DbError, DbResult, ReadTx, WriteTx};
use crate::ReadWriteTx;

pub const MAX_TABLES: u64 = 1024;
pub const MIN_DB_SIZE: isize = 100_000_000;
/// Database size: up to 2,147,483,648 pages (≈8.0 TiB for default 4 KiB pagesize). 
/// Reference: https://github.com/erthink/libmdbx/blob/master/README.md
/// Default page size on Linus is 4 KiB.  
/// Reference: https://docs.kernel.org/admin-guide/mm/transhuge.html
pub const MAX_DB_SIZE: isize = 8_000_000_000_000;
pub const DB_GROWTH_STEP: isize = 100_000_000;

pub struct Mdbx {
    db: libmdbx::Database<WriteMap>,
}

impl Mdbx {
    pub fn open(path: impl AsRef<Path>) -> DbResult<Self> {
        let db_opts = DatabaseOptions {
            mode: libmdbx::Mode::ReadWrite(ReadWriteOptions {
                min_size: Some(MIN_DB_SIZE),
                max_size: Some(MAX_DB_SIZE),
                growth_step: Some(DB_GROWTH_STEP),
                ..Default::default()
            }),
            max_tables: Some(MAX_TABLES),
            ..Default::default()
        };
        let db = libmdbx::Database::open_with_options(path, db_opts).map_err(DbError::custom)?;
        Ok(Self { db })
    }

    pub fn open_with_size<P: AsRef<Path>>(
        path: P,
        min_size: isize,
        max_size: isize,
    ) -> DbResult<Self> {
        let db_opts = DatabaseOptions {
            mode: libmdbx::Mode::ReadWrite(ReadWriteOptions {
                min_size: Some(min_size),
                max_size: Some(max_size),
                growth_step: Some(DB_GROWTH_STEP),
                ..Default::default()
            }),
            max_tables: Some(MAX_TABLES),
            ..Default::default()
        };
        let db = libmdbx::Database::open_with_options(path, db_opts).map_err(DbError::custom)?;
        Ok(Self { db })
    }
}

impl<'a> crate::Database<'a> for Mdbx {
    fn begin_ro(&'a self) -> DbResult<Box<dyn ReadTx + 'a>> {
        let tx = self.db.begin_ro_txn().map_err(DbError::custom)?;
        Ok(Box::new(MdbxTx { tx }))
    }

    fn begin_rw(&'a mut self) -> DbResult<Box<dyn ReadWriteTx + 'a>> {
        let tx = self.db.begin_rw_txn().map_err(DbError::custom)?;
        Ok(Box::new(MdbxTx { tx }))
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

impl<TK: TransactionKind> ReadTx for MdbxTx<'_, TK> {
    fn get(&self, table: &str, key: &[u8]) -> DbResult<Option<Box<[u8]>>> {
        let table = self.get_table(table)?;
        Ok(self
            .tx
            .get::<Vec<u8>>(&table, key.as_ref())
            .map_err(DbError::custom)?
            .map(Vec::into_boxed_slice))
    }
}

impl WriteTx for MdbxTx<'_, RW> {
    fn create_table(&mut self, table: &str) -> DbResult<()> {
        // `create_table` creates only if the table doesn't exist
        self.tx
            .create_table(table.into(), TableFlags::CREATE)
            .map_err(DbError::custom)?;
        Ok(())
    }

    fn insert(&mut self, table: &str, key: &[u8], value: &[u8]) -> DbResult<()> {
        let mdbx_table = self.get_table(table)?;
        self.tx
            .put(&mdbx_table, key, value, WriteFlags::NO_OVERWRITE)
            .map_err(|err| match err {
                libmdbx::Error::KeyExist => DbError::duplicate_key(table, key),
                _ => DbError::custom(err),
            })
    }

    fn upsert(&mut self, table: &str, key: &[u8], value: &[u8]) -> DbResult<()> {
        let mdbx_table = self.get_table(table)?;
        self.tx
            .put(&mdbx_table, key, value, WriteFlags::UPSERT)
            .map_err(DbError::custom)
    }

    fn delete(&mut self, table: &str, key: &[u8]) -> DbResult<()> {
        let mdbx_table = self.get_table(table)?;
        self.tx
            .del(&mdbx_table, key, None)
            .map_err(DbError::custom)?
            .then_some(())
            .ok_or(DbError::non_existing_key(table, key))
    }

    fn commit(self: Box<Self>) -> DbResult<()> {
        self.tx.commit().map_err(DbError::custom)?;
        Ok(())
    }
}

#[cfg(test)]
mod tests;

#[cfg(test)]
mod other_tests {
    use tempfile::tempdir;

    use super::*;
    use crate::{Database, ReadTx};

    #[test]
    fn test_dynamic_resize() -> anyhow::Result<()> {
        use crate::ReadTx;

        let dir = tempfile::tempdir()?;
        let db_path = dir.path().join("testdb");

        let min_size = 0;
        let max_size = 100_000;
        let mut mdbx = Mdbx::open_with_size(&db_path, min_size, max_size)?;

        {
            let mut wtx = mdbx.begin_rw()?;
            wtx.create_table("table1")?;
            wtx.insert("table1", b"key1", b"val1")?;
            wtx.commit()?;
        }

        let result = (|| {
            let mut wtx = mdbx.begin_rw()?;
            let key = b"k0".to_vec();
            let value = vec![0_u8; 100_000];
            wtx.insert("table1", &key, &value)?;
            wtx.commit()
        })();

        if let Err(DbError::Custom(msg)) = result {
            assert!(msg.contains("MDBX_MAP_FULL"));
        } else {
            panic!("expected MapFull error");
        }

        drop(mdbx);

        let new_max_size = 300_000;
        let mut mdbx2 = Mdbx::open_with_size(&db_path, min_size, new_max_size)?;

        {
            let rtx = mdbx2.begin_ro()?;
            let found = rtx.get("table1", b"key1")?;
            assert_eq!(found.as_deref(), Some(b"val1".as_slice()));
        }

        {
            let mut wtx = mdbx2.begin_rw()?;
            wtx.insert("table1", b"key2", &vec![0_u8; 100_000])?;
            wtx.commit()?;
        }

        Ok(())
    }
}

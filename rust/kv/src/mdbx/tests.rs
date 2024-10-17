use std::path::PathBuf;

use anyhow::Result;
use tempfile::{NamedTempFile, TempDir};

use super::*;
use crate::Database;

// Tests use only one table
const TABLE: &str = "table";

// Macro for db setup - `db_dir` needs to be in scope, otherwise the directory would not be cleaned up after test
macro_rules! temp_db {
    ($db_var:ident) => {
        let db_dir = TempDir::new().expect("Failed to create temp dir");
        let mut $db_var = Mdbx::open(db_dir.path()).expect("Failed to open database");
    };
}

fn crate_and_insert(db: &mut Mdbx, key: impl AsRef<[u8]>, value: impl AsRef<[u8]>) -> DbResult<()> {
    let mut tx = db.begin_rw()?;
    tx.create_table(TABLE)?;
    tx.insert(TABLE, key.as_ref(), value.as_ref())?;
    tx.commit();
    Ok(())
}

fn insert(db: &mut Mdbx, key: impl AsRef<[u8]>, value: impl AsRef<[u8]>) -> DbResult<()> {
    let mut tx = db.begin_rw()?;
    tx.insert(TABLE, key.as_ref(), value.as_ref())?;
    tx.commit();
    Ok(())
}

fn upsert(db: &mut Mdbx, key: impl AsRef<[u8]>, value: impl AsRef<[u8]>) -> DbResult<()> {
    let mut tx = db.begin_rw()?;
    tx.upsert(TABLE, key.as_ref(), value.as_ref())?;
    tx.commit();
    Ok(())
}

fn delete(db: &mut Mdbx, key: impl AsRef<[u8]>) -> DbResult<()> {
    let mut tx = db.begin_rw()?;
    tx.delete(TABLE, key.as_ref())?;
    tx.commit();
    Ok(())
}

fn get(db: &Mdbx, key: impl AsRef<[u8]>) -> DbResult<Option<Box<[u8]>>> {
    db.begin_ro()?.get(TABLE, key.as_ref())
}

#[test]
fn db_flow() -> Result<()> {
    temp_db!(db);

    crate_and_insert(&mut db, [0], [1])?;
    assert_eq!(*get(&db, [0])?.unwrap(), [1]);

    delete(&mut db, [0])?;
    assert_eq!(get(&db, [0])?, None);

    Ok(())
}

#[test]
fn get_no_table() {
    temp_db!(db);
    assert_eq!(get(&db, [0]).unwrap_err(), DbError::non_existing_table(TABLE));
}

#[test]
fn get_missing_key() -> Result<()> {
    temp_db!(db);

    crate_and_insert(&mut db, [0], [1])?;
    assert_eq!(get(&db, [2])?, None);

    Ok(())
}

#[test]
fn insert_duplicate_key() -> Result<()> {
    temp_db!(db);

    crate_and_insert(&mut db, [0], [1])?;
    assert_eq!(insert(&mut db, [0], [1]).unwrap_err(), DbError::duplicate_key(TABLE, [0]));

    Ok(())
}

#[test]
fn insert_upsert() -> Result<()> {
    temp_db!(db);

    crate_and_insert(&mut db, [0], [0]);
    upsert(&mut db, [0], [1]);
    assert_eq!(*get(&db, [0])?.unwrap(), [1]);

    Ok(())
}

#[test]
fn delete_no_table() {
    temp_db!(db);
    assert_eq!(delete(&mut db, [0]).unwrap_err(), DbError::non_existing_table(TABLE));
}

#[test]
fn delete_missing_key() -> Result<()> {
    temp_db!(db);

    crate_and_insert(&mut db, [0], [1])?;
    assert_eq!(delete(&mut db, [2]).unwrap_err(), DbError::non_existing_key(TABLE, [2]));

    Ok(())
}

#[test]
fn rollback_on_drop() -> Result<()> {
    temp_db!(db);

    {
        // Insert without commit
        let mut tx = db.begin_rw()?;
        tx.create_table(TABLE)?;
        tx.insert(TABLE, &[0][..], &[1][..])?;
    }

    // Table should not exist
    assert_eq!(get(&db, [0]).unwrap_err(), DbError::non_existing_table(TABLE));

    Ok(())
}

#[test]
fn persistence() -> Result<()> {
    let db_dir = TempDir::new()?;
    let path = PathBuf::from(db_dir.path());
    let mut db = Mdbx::open(&path)?;
    crate_and_insert(&mut db, [0], [1])?;

    // After reopening, the data is still there
    drop(db);
    let db = Mdbx::open(&path)?;
    assert_eq!(*get(&db, [0])?.unwrap(), [1]);

    // But it's gone if we delete the directory
    drop(db);
    drop(db_dir);
    let db = Mdbx::open(&path)?;
    assert_eq!(get(&db, [0]).unwrap_err(), DbError::non_existing_table(TABLE));

    Ok(())
}

use anyhow::Result;
use tempfile::{NamedTempFile, TempDir};

use crate::Database;

use super::*;

fn crate_and_insert(
    db: &mut Mdbx,
    table: &str,
    key: impl AsRef<[u8]>,
    value: impl AsRef<[u8]>,
) -> DbResult<()> {
    db.with_rw_tx(|tx| {
        tx.create_table(table)?;
        tx.insert(table, key, value)
    })
}

#[test]
fn db_flow() -> Result<()> {
    let db_dir = TempDir::new()?;
    let mut db = Mdbx::open(db_dir.path())?;

    let key = [0];
    let value = [1];

    // Check insert
    crate_and_insert(&mut db, "table", key, value)?;
    let read_val = db.with_ro_tx(|tx| tx.get("table", key))?;
    assert_eq!(read_val.unwrap().as_ref(), value);

    // Check delete
    db.with_rw_tx(|tx| tx.delete("table", key))?;
    let read_val = db.with_ro_tx(|tx| tx.get("table", key))?;
    assert!(read_val.is_none());

    Ok(())
}

#[test]
fn get_no_table() -> Result<()> {
    let db_dir = TempDir::new()?;
    let mut db = Mdbx::open(db_dir.path())?;

    let result = db.with_ro_tx(|tx| tx.get("table", [0]));
    assert_eq!(result.unwrap_err(), DbError::NonExistingTable);

    Ok(())
}

#[test]
fn get_missing_key() -> Result<()> {
    let db_dir = TempDir::new()?;
    let mut db = Mdbx::open(db_dir.path())?;

    crate_and_insert(&mut db, "table", [0], [1])?;
    let result = db.with_ro_tx(|tx| tx.get("table", [2]))?;
    assert!(result.is_none());

    Ok(())
}

#[test]
fn insert_duplicate_key() -> Result<()> {
    let db_dir = TempDir::new()?;
    let mut db = Mdbx::open(db_dir.path())?;

    crate_and_insert(&mut db, "table", [0], [1])?;
    let result = db.with_rw_tx(|tx| tx.insert("table", [0], [1]));
    assert_eq!(result.unwrap_err(), DbError::DuplicateKey);

    Ok(())
}

#[test]
fn delete_no_table() -> Result<()> {
    let db_dir = TempDir::new()?;
    let mut db = Mdbx::open(db_dir.path())?;

    let result = db.with_rw_tx(|tx| tx.delete("table", [0]));
    assert_eq!(result.unwrap_err(), DbError::NonExistingTable);

    Ok(())
}

#[test]
fn delete_missing_key() -> Result<()> {
    let db_dir = TempDir::new()?;
    let mut db = Mdbx::open(db_dir.path())?;

    crate_and_insert(&mut db, "table", [0], [1])?;
    let result = db.with_rw_tx(|tx| tx.delete("table", [2]));
    assert_eq!(result.unwrap_err(), DbError::NonExistingKey);
    Ok(())
}

#[test]
fn rollback_on_drop() -> Result<()> {
    let db_dir = TempDir::new()?;
    let mut db = Mdbx::open(db_dir.path())?;

    {
        // Insert without commit
        let mut tx = db.begin_rw()?;
        tx.create_table("table")?;
        tx.insert("table", [0], [1])?;
    }

    // Table should not exist
    let result = db.with_ro_tx(|tx| tx.get("table", [0]));
    assert_eq!(result.unwrap_err(), DbError::NonExistingTable);
    Ok(())
}

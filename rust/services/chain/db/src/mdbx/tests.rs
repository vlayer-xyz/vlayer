use tempfile::{NamedTempFile, TempDir};

use crate::Database;

use super::*;

#[test]
fn test_db_flow() {
    let db_dir = TempDir::new().expect("Failed to create temp file");
    let mut db = Mdbx::open(db_dir.path()).expect("DB open failed");

    let table = "table";
    let key = "key".as_bytes();
    let value = "val".as_bytes();

    {
        // Insert
        let mut tx = db.begin_rw().expect("begin_rw failed");
        tx.insert(table, key, value).expect("insert failed");
        tx.commit().expect("commit failed");
    }

    {
        // Verify insert result
        let tx = db.begin_ro().expect("begin_ro failed");
        let read_val = tx
            .get(table, key)
            .expect("get failed")
            .expect("value missing");
        assert_eq!(value, read_val.as_ref());
    }

    {
        // Delete
        let mut tx = db.begin_rw().expect("begin_rw failed");
        tx.delete(table, key).expect("delete failed");
        tx.commit().expect("commit failed");
    }

    {
        // Verify delete result
        let tx = db.begin_ro().expect("begin_ro failed");
        let read_val = tx.get(table, key).expect("get failed");
        assert!(read_val.is_none())
    }
}

#[test]
fn test_get_no_table() {
    let db_dir = TempDir::new().expect("Failed to create temp file");
    let mut db = Mdbx::open(db_dir.path()).expect("DB open failed");
    let tx = db.begin_ro().expect("begin_ro failed");
    let result = tx.get("table", "key").expect("get failed");
    assert!(result.is_none())
}

#[test]
fn test_get_missing_key() {
    let db_dir = TempDir::new().expect("Failed to create temp file");
    let mut db = Mdbx::open(db_dir.path()).expect("DB open failed");

    let table = "table";
    {
        // Insert some key/value just to init the table
        let mut tx = db.begin_rw().expect("begin_rw failed");
        tx.insert(table, "key", "value").expect("insert failed");
        tx.commit().expect("commit failed");
    }

    {
        // Get another key
        let tx = db.begin_ro().expect("begin_ro failed");
        let result = tx.get(table, "").expect("get failed");
        assert!(result.is_none())
    }
}

#[test]
fn test_insert_duplicate_key() {
    let db_dir = TempDir::new().expect("Failed to create temp file");
    let mut db = Mdbx::open(db_dir.path()).expect("DB open failed");

    let table = "table";
    let key = "key".as_bytes();
    let value = "val".as_bytes();

    {
        // Insert once
        let mut tx = db.begin_rw().expect("begin_rw failed");
        tx.insert(table, key, value).expect("insert failed");
        tx.commit().expect("commit failed");
    }

    {
        // Insert twice
        let mut tx = db.begin_rw().expect("begin_rw failed");
        let result = tx.insert(table, key, value);
        assert!(result.is_err_and(|err| err == DbError::DuplicateKey));
    }
}

#[test]
fn test_delete_no_table() {
    let db_dir = TempDir::new().expect("Failed to create temp file");
    let mut db = Mdbx::open(db_dir.path()).expect("DB open failed");
    let mut tx = db.begin_rw().expect("begin_rw failed");
    let result = tx.delete("table", "key");
    assert!(result.is_err_and(|err| err == DbError::NonExistingKey));
}

#[test]
fn test_delete_missing_key() {
    let db_dir = TempDir::new().expect("Failed to create temp file");
    let mut db = Mdbx::open(db_dir.path()).expect("DB open failed");

    let table = "table";
    {
        // Insert some key/value just to init the table
        let mut tx = db.begin_rw().expect("begin_rw failed");
        tx.insert(table, "key", "value").expect("insert failed");
        tx.commit().expect("commit failed");
    }

    {
        // Delete another key
        let mut tx = db.begin_rw().expect("begin_rw failed");
        let result = tx.delete(table, "");
        assert!(result.is_err_and(|err| err == DbError::NonExistingKey));
    }
}

#[test]
fn test_rollback_on_drop() {
    let db_dir = TempDir::new().expect("Failed to create temp file");
    let mut db = Mdbx::open(db_dir.path()).expect("DB open failed");

    let table = "table";
    let key = "key".as_bytes();
    let value = "val".as_bytes();

    {
        // Insert without commit
        let mut tx = db.begin_rw().expect("begin_rw failed");
        tx.insert(table, key, value).expect("insert failed");
    }

    {
        // Value should not exist
        let tx = db.begin_ro().expect("begin_ro failed");
        let result = tx.get(table, key).expect("get failed");
        assert!(result.is_none());
    }
}

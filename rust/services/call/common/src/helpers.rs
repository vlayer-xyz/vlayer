#![allow(clippy::disallowed_types)]
use std::fmt;

use revm::{Database, DatabaseRef, db::WrapDatabaseRef};

pub trait RevmDBError:
    std::error::Error + fmt::Debug + fmt::Display + Send + Sync + 'static + Into<anyhow::Error>
{
}
impl<T> RevmDBError for T where
    T: std::error::Error + fmt::Debug + fmt::Display + Send + Sync + 'static + Into<anyhow::Error>
{
}

/// Helper trait to be used in bounds instead of revm::DatabaseRef
/// Requires Database and it's errors to be Debug, anyhow wrappable and safe in async contexts
pub trait RevmDB: DatabaseRef<Error: RevmDBError> + Send + Sync + fmt::Debug {
    type Error: RevmDBError;
}

impl<T, E> RevmDB for T
where
    T: DatabaseRef<Error = E> + Send + Sync + fmt::Debug,
    E: RevmDBError,
{
    type Error = E;
}

pub type WrappedRevmDBError<D> = <WrapDatabaseRef<D> as Database>::Error;

pub trait WrappedRevmDB:
    Database<Error: fmt::Debug + Send + Sync + 'static + Into<anyhow::Error>> + Send + Sync + fmt::Debug
{
    type Error: RevmDBError;
}

impl<T, E> WrappedRevmDB for WrapDatabaseRef<T>
where
    T: DatabaseRef<Error = E> + Send + Sync + fmt::Debug,
    E: RevmDBError,
{
    type Error = E;
}

#![allow(clippy::disallowed_types)]
use std::fmt;

use revm::DatabaseRef;

/// Helper trait to be used in bounds instead of revm::DatabaseRef
/// Requires Database and it's errors to be Debug, anyhow wrappable and safe in async contexts
pub trait RevmDB:
    DatabaseRef<Error: fmt::Debug + Send + Sync + 'static + Into<anyhow::Error>>
    + Send
    + Sync
    + fmt::Debug
{
    type Error: fmt::Debug + Send + Sync + 'static + Into<anyhow::Error>;
}

impl<T, E> RevmDB for T
where
    T: DatabaseRef<Error = E> + Send + Sync + fmt::Debug,
    E: fmt::Debug + Send + Sync + 'static + Into<anyhow::Error>,
{
    type Error = E;
}

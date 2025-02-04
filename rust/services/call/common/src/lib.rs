#![allow(clippy::disallowed_types)]
use std::fmt;

use revm::DatabaseRef;

pub trait Database:
    DatabaseRef<Error: fmt::Debug + Send + Sync + 'static + Into<anyhow::Error>>
    + Send
    + Sync
    + fmt::Debug
{
    type Error: fmt::Debug + Send + Sync + 'static + Into<anyhow::Error>;
}

impl<T, E> Database for T
where
    T: DatabaseRef<Error = E> + Send + Sync + fmt::Debug,
    E: fmt::Debug + Send + Sync + 'static + Into<anyhow::Error>,
{
    type Error = E;
}

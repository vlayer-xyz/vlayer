use derivative::Derivative;
use revm::DatabaseRef;
use thiserror::Error;

use super::{location::ExecutionLocation, EvmEnv};

#[derive(Debug, Error, Derivative)]
#[derivative(PartialEq, Eq)]
pub enum Error {
    #[error(transparent)]
    Opaque(
        #[from]
        #[derivative(PartialEq = "ignore")]
        anyhow::Error,
    ),
    #[error("NullEvmEnvFactory cannot create EvmEnv")]
    NullEvmEnvFactory,
}
pub type Result<T> = std::result::Result<T, Error>;

pub trait EvmEnvFactory<D>: Send + Sync
where
    D: DatabaseRef + Send + Sync,
{
    fn create(&self, location: ExecutionLocation) -> Result<EvmEnv<D>>;
}

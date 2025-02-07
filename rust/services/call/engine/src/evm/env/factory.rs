use call_common::{ExecutionLocation, RevmDB};
use derivative::Derivative;
use thiserror::Error;

use super::EvmEnv;

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

pub trait EvmEnvFactory<D: RevmDB>: Send + Sync {
    fn create(&self, location: ExecutionLocation) -> Result<EvmEnv<D>>;
}

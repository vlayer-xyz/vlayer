use std::any::Any;

use revm::primitives::EVMError;

use crate::evm::execution_result::TransactError;

#[derive(thiserror::Error, Debug, PartialEq)]
pub enum Error {
    #[error("EVM transact preverified error: {0}")]
    TransactPreverifiedError(String),

    #[error("EVM transact error: {0}")]
    TransactError(#[from] TransactError),

    #[error("Failed to get EvmEnv: {0}")]
    EvmEnv(#[from] crate::evm::env::factory::Error),

    #[error("Panic: {0}")]
    Panic(String),
}

pub type Result<T> = std::result::Result<T, Error>;

impl<D: std::fmt::Debug> From<EVMError<D>> for Error {
    fn from(err: EVMError<D>) -> Self {
        match err {
            EVMError::Precompile(err) => TransactError::Revert(err).into(),
            _ => Error::TransactPreverifiedError(format!("{err:?}")),
        }
    }
}

pub fn wrap_panic(err: Box<dyn Any + Send>) -> Error {
    let panic_msg = err
        .downcast::<String>()
        .map(|x| *x)
        .unwrap_or("Panic occurred".to_string());
    Error::Panic(panic_msg)
}

use std::any::Any;

use call_common::RevmDBError;
use revm::primitives::EVMError;

use crate::evm::execution_result::TransactError;

#[derive(thiserror::Error, Debug, PartialEq)]
pub enum Error<D: RevmDBError> {
    #[error("EVM error: {0}")]
    EvmError(#[from] EVMError<D>),

    #[error("EVM transact error: {0}")]
    TransactError(#[from] TransactError),

    #[error("Failed to get EvmEnv: {0}")]
    EvmEnv(#[from] crate::evm::env::factory::Error),

    #[error("Panic: {0}")]
    Panic(String),
}

pub type Result<T, D> = std::result::Result<T, Error<D>>;

pub fn wrap_panic<D: RevmDBError>(err: Box<dyn Any + Send>) -> Error<D> {
    let panic_msg = err
        .downcast::<String>()
        .map(|x| *x)
        .unwrap_or("Panic occurred".to_string());
    Error::Panic(panic_msg)
}

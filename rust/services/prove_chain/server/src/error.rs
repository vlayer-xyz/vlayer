use axum_jrpc::{
    error::{JsonRpcError, JsonRpcErrorReason},
    Value,
};
use mpt::MptError;
use prove_chain_host::HostError;
use server_utils::FieldValidationError;
use thiserror::Error;

#[derive(Debug, Error, PartialEq)]
pub enum AppError {
    #[error("Invalid params: empty list of block numbers provided - nothing to prove")]
    NoBlockNumbers,
    #[error("Invalid field: {0}")]
    FieldValidation(#[from] FieldValidationError),

    #[error("Host error: {0}")]
    Host(#[from] HostError),

    #[error("Bincode error: {0}")]
    Bincode(String),

    #[error("MPT error: {0}")]
    Mpt(#[from] MptError),
}

impl From<AppError> for JsonRpcError {
    fn from(error: AppError) -> Self {
        match error {
            AppError::Bincode(..)
            | AppError::Host(..)
            | AppError::Mpt(..)
            | AppError::NoBlockNumbers
            | AppError::FieldValidation(..) => JsonRpcError::new(
                JsonRpcErrorReason::InvalidParams,
                error.to_string(),
                Value::Null,
            ),
        }
    }
}

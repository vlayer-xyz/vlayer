use axum_jrpc::{
    error::{JsonRpcError, JsonRpcErrorReason},
    Value,
};
use server_utils::FieldValidationError;
use thiserror::Error;

#[derive(Debug, Error, PartialEq)]
pub enum AppError {
    #[error("Invalid params: empty list of block hashes provided - nothing to prove")]
    NoBlockHashes,
    #[error("Invalid field: {0}")]
    FieldValidation(#[from] FieldValidationError),
}

impl From<AppError> for JsonRpcError {
    fn from(error: AppError) -> Self {
        match error {
            AppError::NoBlockHashes | AppError::FieldValidation(..) => JsonRpcError::new(
                JsonRpcErrorReason::InvalidParams,
                error.to_string(),
                Value::Null,
            ),
        }
    }
}

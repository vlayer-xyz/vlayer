use axum_jrpc::{
    error::{JsonRpcError, JsonRpcErrorReason},
    Value,
};
use thiserror::Error;

#[derive(Debug, Error, PartialEq)]
pub enum AppError {
    #[error("Invalid params: empty list of block hashes provided - nothing to prove")]
    NoBlockHashes,
}

impl From<AppError> for JsonRpcError {
    fn from(error: AppError) -> Self {
        match error {
            AppError::NoBlockHashes => JsonRpcError::new(
                JsonRpcErrorReason::InvalidParams,
                error.to_string(),
                Value::Null,
            ),
        }
    }
}

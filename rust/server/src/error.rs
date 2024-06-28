use axum_jrpc::{
    error::{JsonRpcError, JsonRpcErrorReason},
    Value,
};
use hex::FromHexError;
use host::host::HostError;
use thiserror::Error;

#[derive(Debug, Error, PartialEq)]
pub enum FieldValidationError {
    #[error("{1} `{0}`")]
    InvalidHex(String, FromHexError),
    #[error("Invalid hex prefix `{0}`")]
    InvalidHexPrefix(String),
}

#[derive(Debug, Error)]
pub enum AppError {
    #[error("Invalid field `{0}`: {1}")]
    FieldValidationError(String, FieldValidationError),
    #[error("Host error: {0}")]
    HostError(#[from] HostError),
}

impl From<AppError> for JsonRpcError {
    fn from(error: AppError) -> Self {
        match error {
            AppError::FieldValidationError(..) => JsonRpcError::new(
                JsonRpcErrorReason::InvalidParams,
                error.to_string(),
                Value::Null,
            ),
            AppError::HostError(..) => JsonRpcError::new(
                JsonRpcErrorReason::InternalError,
                error.to_string(),
                Value::Null,
            ),
        }
    }
}

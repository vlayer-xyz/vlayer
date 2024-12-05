use axum_jrpc::{
    error::{JsonRpcError, JsonRpcErrorReason},
    Value,
};
use call_host::Error as HostError;
use server_utils::{rpc::Error as RpcError, FieldValidationError};
use thiserror::Error;
use tokio::task::JoinError;

#[derive(Debug, Error)]
pub enum AppError {
    #[error("Invalid field: {0}")]
    FieldValidation(#[from] FieldValidationError),
    #[error("Host error: {0}")]
    Host(#[from] HostError),
    #[error("Join error: {0}")]
    Join(#[from] JoinError),
    #[error("RPC error: {0}")]
    RpcError(#[from] RpcError),
    #[error("Hash not found: {0}")]
    HashNotFound(String),
}

impl From<AppError> for JsonRpcError {
    fn from(error: AppError) -> Self {
        match error {
            AppError::FieldValidation(..) => {
                JsonRpcError::new(JsonRpcErrorReason::InvalidParams, error.to_string(), Value::Null)
            }
            AppError::Host(..)
            | AppError::Join(..)
            | AppError::RpcError(..)
            | AppError::HashNotFound(..) => {
                JsonRpcError::new(JsonRpcErrorReason::InternalError, error.to_string(), Value::Null)
            }
        }
    }
}

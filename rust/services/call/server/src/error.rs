use call_host::Error as HostError;
use jsonrpsee::types::error::{self as jrpcerror, ErrorObjectOwned};
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
    #[error("Waiting for chain proof timed out")]
    ChainProofTimeout,
}

impl From<AppError> for ErrorObjectOwned {
    fn from(error: AppError) -> Self {
        match error {
            AppError::FieldValidation(..) => ErrorObjectOwned::owned::<()>(
                jrpcerror::INVALID_PARAMS_CODE,
                error.to_string(),
                None,
            ),
            AppError::Host(..)
            | AppError::Join(..)
            | AppError::RpcError(..)
            | AppError::HashNotFound(..)
            | AppError::ChainProofTimeout => ErrorObjectOwned::owned::<()>(
                jrpcerror::INTERNAL_ERROR_CODE,
                error.to_string(),
                None,
            ),
        }
    }
}

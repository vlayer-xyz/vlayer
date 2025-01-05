use std::num::TryFromIntError;

use call_host::Error as HostError;
use jsonrpsee::types::error::{self as jrpcerror, ErrorObjectOwned};
use server_utils::{rpc::Error as RpcError, FieldValidationError};
use thiserror::Error;
use tokio::task::JoinError;

use crate::v_call::CallHash;

#[derive(Debug, Error)]
pub enum AppError {
    #[error("Invalid field: {0}")]
    FieldValidation(#[from] FieldValidationError),
    #[error("Host error: {0}")]
    Host(#[from] HostError),
    #[error("Seal error: {0}")]
    Seal(#[from] seal::Error),
    #[error("Join error: {0}")]
    Join(#[from] JoinError),
    #[error("RPC error: {0}")]
    RpcError(#[from] RpcError),
    #[error("Hash not found: {0}")]
    HashNotFound(CallHash),
    #[error("Waiting for chain proof timed out")]
    ChainProofTimeout,
    #[error("Int conversion error: {0}")]
    TryFromInt(#[from] TryFromIntError),
}

impl From<AppError> for ErrorObjectOwned {
    fn from(error: AppError) -> Self {
        (&error).into()
    }
}

impl From<&AppError> for ErrorObjectOwned {
    fn from(error: &AppError) -> Self {
        match error {
            AppError::FieldValidation(..) => ErrorObjectOwned::owned::<()>(
                jrpcerror::INVALID_PARAMS_CODE,
                error.to_string(),
                None,
            ),
            AppError::HashNotFound(..) => ErrorObjectOwned::owned::<()>(
                jrpcerror::INVALID_REQUEST_CODE,
                error.to_string(),
                None,
            ),
            AppError::Host(..)
            | AppError::Seal(..)
            | AppError::Join(..)
            | AppError::RpcError(..)
            | AppError::ChainProofTimeout
            | AppError::TryFromInt(..) => ErrorObjectOwned::owned::<()>(
                jrpcerror::INTERNAL_ERROR_CODE,
                error.to_string(),
                None,
            ),
        }
    }
}

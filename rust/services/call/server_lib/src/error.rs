use std::num::TryFromIntError;

use call_host::Error as HostError;
use jsonrpsee::types::error::{self as jrpcerror, ErrorObjectOwned};
use server_utils::{rpc::Error as RpcError, FieldValidationError};
use thiserror::Error;
use tokio::task::JoinError;

use crate::v_call::CallHash;

#[derive(Debug, Error)]
pub enum ChainProofError {
    #[error("Waiting for chain proof timed out")]
    Timeout,
    #[error("Host error: {0}")]
    Host(#[from] HostError),
}

#[derive(Debug, Error)]
pub enum GasMeterError {
    #[error("RPC error: {0}")]
    Rpc(#[from] RpcError),
}

#[derive(Debug, Error)]
pub enum MetricsError {
    #[error("Int conversion error: {0}")]
    TryFromInt(#[from] TryFromIntError),
}

#[derive(Debug, Error)]
pub enum PreflightError {
    #[error("Host error: {0}")]
    Host(#[from] HostError),
    #[error("Gas meter error: {0}")]
    GasMeter(#[from] GasMeterError),
    #[error("Metrics error: {0}")]
    Metrics(#[from] MetricsError),
}

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
    #[error("Int conversion error: {0}")]
    TryFromInt(#[from] TryFromIntError),
    #[error("Gas meter error: {0}")]
    GasMeter(#[from] GasMeterError),
    #[error("Chain proof error: {0}")]
    ChainProof(#[from] ChainProofError),
    #[error("Preflight error: {0}")]
    Preflight(#[from] PreflightError),
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
            | AppError::ChainProof(..)
            | AppError::TryFromInt(..)
            | AppError::GasMeter(..)
            | AppError::Preflight(..) => ErrorObjectOwned::owned::<()>(
                jrpcerror::INTERNAL_ERROR_CODE,
                error.to_string(),
                None,
            ),
        }
    }
}

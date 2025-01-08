use call_host::Error as HostError;
use jsonrpsee::types::error::{self as jrpcerror, ErrorObjectOwned};
use server_utils::rpc::Error as RpcError;
use thiserror::Error;
use tokio::task::JoinError;

use crate::{
    chain_proof::Error as ChainProofError, preflight::Error as PreflightError,
    proving::Error as ProvingError, v_call::CallHash,
};

#[derive(Debug, Error)]
pub enum AppError {
    #[error("Host error: {0}")]
    Host(#[from] HostError),
    #[error("Join error: {0}")]
    Join(#[from] JoinError),
    #[error("RPC error: {0}")]
    RpcError(#[from] RpcError),
    #[error("Hash not found: {0}")]
    HashNotFound(CallHash),
    #[error("Chain proof error: {0}")]
    ChainProof(#[from] ChainProofError),
    #[error("Preflight error: {0}")]
    Preflight(#[from] PreflightError),
    #[error("Proving error: {0}")]
    Proving(#[from] ProvingError),
}

impl From<AppError> for ErrorObjectOwned {
    fn from(error: AppError) -> Self {
        (&error).into()
    }
}

impl From<&AppError> for ErrorObjectOwned {
    fn from(error: &AppError) -> Self {
        match error {
            AppError::HashNotFound(..) => ErrorObjectOwned::owned::<()>(
                jrpcerror::INVALID_REQUEST_CODE,
                error.to_string(),
                None,
            ),
            AppError::Host(..)
            | AppError::Join(..)
            | AppError::RpcError(..)
            | AppError::ChainProof(..)
            | AppError::Preflight(..)
            | AppError::Proving(..) => ErrorObjectOwned::owned::<()>(
                jrpcerror::INTERNAL_ERROR_CODE,
                error.to_string(),
                None,
            ),
        }
    }
}

use axum_jrpc::{
    error::{JsonRpcError, JsonRpcErrorReason},
    Value,
};
use prove_chain_host::HostError;
use thiserror::Error;

#[derive(Debug, Error, PartialEq)]
pub enum AppError {
    #[error("Invalid params: empty list of block hashes provided - nothing to prove")]
    NoBlockHashes,

    #[error("Host error: {0}")]
    Host(#[from] HostError),

    #[error("Bincode error: {0}")]
    Bincode(String),
}

impl From<AppError> for JsonRpcError {
    fn from(error: AppError) -> Self {
        match error {
            AppError::Bincode(..) | AppError::Host(..) | AppError::NoBlockHashes => {
                JsonRpcError::new(
                    JsonRpcErrorReason::InvalidParams,
                    error.to_string(),
                    Value::Null,
                )
            }
        }
    }
}

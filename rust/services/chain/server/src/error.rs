use std::ops::RangeInclusive;

use alloy_primitives::{BlockNumber, ChainId};
use axum_jrpc::{
    error::{JsonRpcError, JsonRpcErrorReason},
    Value,
};
use chain_db::ChainDbError;
use mpt::MptError;
use server_utils::FieldValidationError;
use thiserror::Error;

#[derive(Debug, Error, PartialEq)]
pub enum AppError {
    #[error("Invalid params: empty list of block numbers provided - nothing to prove")]
    NoBlockNumbers,
    #[error("Block number {block_num} outside stored range: {block_range:?}")]
    BlockNumberOutsideRange {
        block_num: BlockNumber,
        block_range: RangeInclusive<BlockNumber>,
    },
    #[error("Unsupported chain ID: {0}")]
    UnsupportedChainId(ChainId),
    #[error("Invalid field: {0}")]
    FieldValidation(#[from] FieldValidationError),
    #[error("MPT error: {0}")]
    Mpt(#[from] MptError),
    #[error("Chain db error: {0}")]
    ChainDb(#[from] ChainDbError),
}

impl From<AppError> for JsonRpcError {
    fn from(error: AppError) -> Self {
        match error {
            AppError::NoBlockNumbers
            | AppError::BlockNumberOutsideRange { .. }
            | AppError::UnsupportedChainId(..)
            | AppError::FieldValidation(..) => {
                JsonRpcError::new(JsonRpcErrorReason::InvalidParams, error.to_string(), Value::Null)
            }
            AppError::Mpt(..) | AppError::ChainDb(..) => {
                JsonRpcError::new(JsonRpcErrorReason::InternalError, error.to_string(), Value::Null)
            }
        }
    }
}

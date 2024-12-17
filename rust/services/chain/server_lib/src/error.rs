use alloy_primitives::{BlockNumber, ChainId};
use chain_db::ChainDbError;
use jsonrpsee::types::error::{self as jrpcerror, ErrorObjectOwned};
use mpt::MptError;
use server_utils::FieldValidationError;
use thiserror::Error;
use u64_range::NonEmptyRange;

#[derive(Debug, Error, PartialEq)]
pub enum AppError {
    #[error("Invalid params: empty list of block numbers provided - nothing to prove")]
    NoBlockNumbers,
    #[error("Block number {block_num} outside stored range: {block_range:?}")]
    BlockNumberOutsideRange {
        block_num: BlockNumber,
        block_range: NonEmptyRange,
    },
    #[error("Unsupported chain ID: {0}")]
    UnsupportedChainId(ChainId),
    #[error("Invalid field: {0}")]
    FieldValidation(#[from] FieldValidationError),
    #[error("MPT error: {0}")]
    Mpt(#[from] MptError),
    #[error("Chain db error: {0}")]
    ChainDb(ChainDbError),
}

impl From<ChainDbError> for AppError {
    fn from(err: ChainDbError) -> Self {
        match err {
            ChainDbError::ChainNotFound(chain_id) => Self::UnsupportedChainId(chain_id),
            ChainDbError::BlockNumberOutsideRange {
                block_num,
                block_range,
            } => Self::BlockNumberOutsideRange {
                block_num,
                block_range,
            },
            err => Self::ChainDb(err),
        }
    }
}

impl From<AppError> for ErrorObjectOwned {
    fn from(error: AppError) -> Self {
        match error {
            AppError::NoBlockNumbers
            | AppError::BlockNumberOutsideRange { .. }
            | AppError::UnsupportedChainId(..)
            | AppError::FieldValidation(..) => ErrorObjectOwned::owned::<()>(
                jrpcerror::INVALID_PARAMS_CODE,
                error.to_string(),
                None,
            ),
            AppError::Mpt(..) | AppError::ChainDb(..) => ErrorObjectOwned::owned::<()>(
                jrpcerror::INTERNAL_ERROR_CODE,
                error.to_string(),
                None,
            ),
        }
    }
}

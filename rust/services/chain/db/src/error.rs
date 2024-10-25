use std::ops::RangeInclusive;

use alloy_primitives::{BlockNumber, ChainId};
use thiserror::Error;

#[derive(Error, Debug, PartialEq)]
pub enum ChainDbError {
    #[error("Attempted write on read-only database")]
    ReadOnly,
    #[error("Database error: {0}")]
    Db(#[from] key_value::DbError),
    #[error("RLP error: {0}")]
    Node(#[from] alloy_rlp::Error),
    #[error("Node not found")]
    NodeNotFound,
    #[error("Invalid node")]
    InvalidNode,
    #[error("Block not found")]
    BlockNotFound,
    #[error("ZK proof verification failed: {0}")]
    ZkProofVerificationFailed(#[from] block_trie::ProofVerificationError),
    #[error("Chain not found: {0}")]
    ChainNotFound(ChainId),
    #[error("Block number {block_num} outside stored range: {block_range:?}")]
    BlockNumberOutsideRange {
        block_num: BlockNumber,
        block_range: RangeInclusive<BlockNumber>,
    },
}

pub type ChainDbResult<T> = Result<T, ChainDbError>;

use alloy_primitives::{BlockNumber, ChainId};
use derivative::Derivative;
use thiserror::Error;
use u64_range::NonEmptyRange;

#[derive(Error, Debug, Derivative)]
#[derivative(PartialEq)]
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
    ZkProofVerificationFailed(#[from] chain_common::verifier::Error),
    #[error("Chain not found: {0}")]
    ChainNotFound(ChainId),
    #[error("Block number {block_num} outside stored range: {block_range:?}")]
    BlockNumberOutsideRange {
        block_num: BlockNumber,
        block_range: NonEmptyRange,
    },
    #[error("Malformed proof: {0}")]
    MalformedProof(
        #[from]
        #[derivative(PartialEq = "ignore")]
        bincode::Error,
    ),
}

pub type ChainDbResult<T> = Result<T, ChainDbError>;

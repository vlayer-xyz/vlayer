use std::array::TryFromSliceError;

use derivative::Derivative;
use ethers::types::BlockNumber;
use thiserror::Error;

use super::prover::Error as ProverError;

#[derive(Error, Debug, Derivative)]
#[derivative(PartialEq, Eq)]
pub enum HostError {
    #[error("ChainDB error: {0}")]
    ChainDb(#[from] chain_db::ChainDbError),
    #[error("Prover error: {0}")]
    Prover(#[from] ProverError),
    #[error("Provider: {0}")]
    Provider(String),
    #[error("BlockNotFound: {0}")]
    BlockNotFound(BlockNumber),
    #[error("Block conversion error: {0}")]
    BlockConversion(String),
    #[error("Digest conversion error: {0}")]
    DigestConversion(
        #[from]
        #[derivative(PartialEq = "ignore")]
        TryFromSliceError,
    ),
    #[error("Block trie error: {0}")]
    BlockTrieError(#[from] block_trie::BlockTrieError),
}

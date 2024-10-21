use std::array::TryFromSliceError;

use derivative::Derivative;
use ethers::types::BlockNumber;
use thiserror::Error;

#[derive(Error, Debug, Derivative)]
#[derivative(PartialEq, Eq)]
pub enum HostError {
    #[error("ChainDB error: {0}")]
    ChainDb(#[from] chain_db::ChainDbError),
    #[error("ExecutorEnvBuilder: {0}")]
    ExecutorEnvBuilder(String),
    #[error("Prover: {0}")]
    Prover(String),
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

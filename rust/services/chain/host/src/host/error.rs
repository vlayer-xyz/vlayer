use derivative::Derivative;
use std::array::TryFromSliceError;
use thiserror::Error;

#[derive(Error, Debug, Clone, Derivative)]
#[derivative(PartialEq)]
pub enum HostError {
    #[error("ExecutorEnvBuilder: {0}")]
    ExecutorEnvBuilder(String),
    #[error("Prover: {0}")]
    Prover(String),
    #[error("Provider: {0}")]
    Provider(String),
    #[error("Failed to fetch latest block")]
    NoLatestBlock,
    #[error("Block conversion error: {0}")]
    BlockConversion(String),
    #[error("Digest conversion error: {0}")]
    DigestConversion(
        #[from]
        #[derivative(PartialEq = "ignore")]
        TryFromSliceError,
    ),
}

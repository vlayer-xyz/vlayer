pub use block_trie::BlockTrieError;
pub use chain_db::ChainDbError;
use derivative::Derivative;
use thiserror::Error;

pub use super::{block_fetcher::BlockFetcherError, prover::Error as ProverError};

#[derive(Error, Debug, Derivative)]
#[derivative(PartialEq, Eq)]
pub enum HostError {
    #[error("ChainDB error: {0}")]
    ChainDb(#[from] ChainDbError),
    #[error("Prover error: {0}")]
    Prover(#[from] ProverError),
    #[error("BlockTrie error: {0}")]
    BlockTrieError(#[from] BlockTrieError),
    #[error("Proof serialization error: {0}")]
    ProofSerializationError(
        #[from]
        #[derivative(PartialEq = "ignore")]
        bincode::Error,
    ),
    #[error("BlockFetcher error: {0}")]
    BlockFetcher(#[from] BlockFetcherError),
}

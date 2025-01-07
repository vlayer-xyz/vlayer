use derivative::Derivative;
use thiserror::Error;

use crate::prover::Error as ProverError;

#[derive(Debug, Error, Derivative)]
#[derivative(PartialEq, Eq)]
pub enum Error {
    #[error("Prover: {0}")]
    Prover(#[from] ProverError),
    #[error("ExecutorEnvBuilder: {0}")]
    ExecutorEnvBuilder(
        #[from]
        #[derivative(PartialEq = "ignore")]
        anyhow::Error,
    ),
}
pub type Result<T> = std::result::Result<T, Error>;

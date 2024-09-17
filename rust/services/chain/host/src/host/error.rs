use thiserror::Error;

#[derive(Error, Debug, Clone, PartialEq)]
pub enum HostError {
    #[error("ExecutorEnvBuilder: {0}")]
    ExecutorEnvBuilder(String),
    #[error("Prover: {0}")]
    Prover(String),
}

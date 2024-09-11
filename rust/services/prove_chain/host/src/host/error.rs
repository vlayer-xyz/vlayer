use thiserror::Error;

#[derive(Error, Debug)]
pub enum HostError {
    #[error("ExecutorEnvBuilder: {0}")]
    ExecutorEnvBuilder(String),
    #[error("Prover: {0}")]
    Prover(String),
}

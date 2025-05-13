use call_engine::{GuestOutputError, evm::env::factory::Error as EvmEnvFactoryError};
use host_utils::{ProverError, proving};
use thiserror::Error;

pub mod preflight;

#[derive(Error, Debug)]
pub enum Error {
    #[error("Builder: {0}")]
    Builder(#[from] BuilderError),

    #[error("Preflight: {0}")]
    Preflight(#[from] preflight::Error),

    #[error("Proving: {0}")]
    Proving(#[from] ProvingError),
}

#[derive(Error, Debug)]
pub enum BuilderError {
    #[error("Multi provider error: {0}")]
    MultiProvider(#[from] provider::multi::Error),

    #[error("Fork error: {0}")]
    Fork(#[from] chain::ForkError),

    #[error("Evm env factory error: {0}")]
    EvmEnvFactory(#[from] EvmEnvFactoryError),

    #[error("Chain Proof Client error: {0}")]
    ChainProofClient(#[from] chain_client::Error),

    #[error("Prover contract not deployed")]
    ProverContractNotDeployed,

    #[error("Prover: {0}")]
    Prover(#[from] ProverError),
}

#[derive(Error, Debug)]
pub enum ProvingError {
    #[error("Proving error: {0}")]
    Proving(#[from] proving::Error),

    #[error("Host output does not match guest output: {0:?} {1:?}")]
    HostGuestOutputMismatch(Vec<u8>, Vec<u8>),

    #[error("Guest output error: {0}")]
    GuestOutput(#[from] GuestOutputError),

    #[error("Seal encoding error: {0}")]
    SealEncodingError(#[from] seal::Error),
}

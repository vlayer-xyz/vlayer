use call_engine::{
    evm::env::factory::Error as EvmEnvFactoryError, travel_call, verifier, GuestOutputError,
};
use host_utils::{proving, ProverError};
use risc0_zkp::verify::VerificationError;
use thiserror::Error;

use crate::{db::HostDbError, into_input};

#[derive(Error, Debug)]
pub enum Error {
    #[error("Builder: {0}")]
    Builder(#[from] BuilderError),

    #[error("Proving: {0}")]
    AwaitingChainProof(#[from] AwaitingChainProofError),

    #[error("Preflight: {0}")]
    Preflight(#[from] PreflightError),

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
pub enum AwaitingChainProofError {
    #[error("Chain Proof Client error: {0}")]
    ChainProofClient(#[from] chain_client::Error),
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

#[derive(Error, Debug)]
pub enum PreflightError {
    #[error("Calldata too large: {0} bytes")]
    CalldataTooLargeError(usize),

    #[error("TravelCallExecutor error: {0}")]
    Engine(#[from] travel_call::Error<HostDbError>),

    #[error("Verification error: {0}")]
    Verification(#[from] VerificationError),

    #[error("Creating input: {0}")]
    CreatingInput(#[from] into_input::Error),

    #[error("Travel call verification error: {0}")]
    TravelCall(#[from] verifier::travel_call::Error),
}

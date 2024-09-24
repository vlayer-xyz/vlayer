use alloy_primitives::ChainId;
use call_engine::{engine::EngineError, io::GuestOutputError};
use provider::ProviderFactoryError;
use risc0_zkp::verify::VerificationError;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum HostError {
    #[error("ExecutorEnv builder error")]
    ExecutorEnvBuilder(String),

    #[error("Invalid input")]
    CreatingInput(String),

    #[error("Engine error: {0}")]
    Engine(#[from] EngineError),

    #[error("Provider factory error: {0}")]
    ProviderFactory(#[from] ProviderFactoryError),

    #[error("Provider error: {0}")]
    Provider(String),

    #[error("Block not found: {0}")]
    BlockNotFound(u64),

    #[error("Error creating client: {0}")]
    NewClient(#[from] url::ParseError),

    #[error("Prover error: {0}")]
    Prover(String),

    #[error("Guest output error: {0}")]
    GuestOutput(#[from] GuestOutputError),

    #[error("Host output does not match guest output: {0:?} {1:?}")]
    HostGuestOutputMismatch(Vec<u8>, Vec<u8>),

    #[error("No rpc url for chain: {0}")]
    NoRpcUrl(ChainId),

    #[error("Verification error: {0}")]
    Verification(#[from] VerificationError),

    #[error("Abi encode error: {0}")]
    AbiEncode(String),

    #[error("No rpc cache for chain: {0}")]
    NoRpcCache(ChainId),

    #[error("Evm env factory error: {0}")]
    EvmEnvFactory(#[from] anyhow::Error),

    #[error("Seal encoding error: {0}")]
    SealEncodingError(String),
}

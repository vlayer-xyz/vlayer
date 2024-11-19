use alloy_primitives::ChainId;
use call_engine::{
    travel_call_executor::Error as TravelCallExecutorError, verifier::guest_input, GuestOutputError,
};
use provider::ProviderFactoryError;
use risc0_zkp::verify::VerificationError;
use thiserror::Error;

use super::prover;

#[derive(Error, Debug)]
pub enum Error {
    #[error("Invalid input")]
    CreatingInput(String),

    #[error("TravelCallExecutor error: {0}")]
    Engine(#[from] TravelCallExecutorError),

    #[error("Provider factory error: {0}")]
    ProviderFactory(#[from] ProviderFactoryError),

    #[error("Provider error: {0}")]
    Provider(String),

    #[error("Block not found: {0}")]
    BlockNotFound(u64),

    #[error("Error creating client: {0}")]
    NewClient(#[from] url::ParseError),

    #[error("Prover error: {0}")]
    Prover(#[from] prover::Error),

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

    #[error("Chain Proof Client error: {0}")]
    ChainProofClient(#[from] chain_client::Error),

    #[error("Calldata too large: {0} bytes")]
    CalldataTooLargeError(usize),

    #[error("Guest input verification error: {0}")]
    GuestInput(#[from] guest_input::Error),

    #[error("Could not calculate return hash: {0}")]
    ReturnHashEncoding(String),
}

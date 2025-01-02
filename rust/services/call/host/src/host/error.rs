use call_engine::{
    evm::env::factory::Error as EvmEnvFactoryError,
    travel_call_executor::Error as TravelCallExecutorError, verifier::guest_input,
    GuestOutputError,
};
use ethers_core::types::BlockNumber as BlockTag;
use host_utils::proving;
use provider::ProviderFactoryError;
use risc0_zkp::verify::VerificationError;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum Error {
    #[error("Builder: {0}")]
    Builder(#[from] BuilderError),

    #[error("Proving: {0}")]
    AwaitingChainProof(#[from] AwaitingChainProofError),

    #[error("Preflight: {0}")]
    Preflight(#[from] PreflightError),
<<<<<<< HEAD
=======

    #[error("TravelCallExecutor error: {0}")]
    Engine(#[from] TravelCallExecutorError), // TODO Remove
>>>>>>> 69828ec7 (Split out PreflightError)

    #[error("Proving: {0}")]
    Proving(#[from] ProvingError),
}

#[derive(Error, Debug)]
pub enum BuilderError {
    #[error("Provider factory error: {0}")]
    ProviderFactory(#[from] ProviderFactoryError),

    #[error("Provider error: {0}")]
    Provider(#[from] provider::Error),

    #[error("Block not found: {0}")]
    BlockNotFound(BlockTag),

    #[error("Chain error: {0}")]
    Chain(#[from] chain::Error),

<<<<<<< HEAD
=======
    #[error("Abi encode error: {0}")]
    AbiEncode(String),

>>>>>>> 69828ec7 (Split out PreflightError)
    #[error("Evm env factory error: {0}")]
    EvmEnvFactory(#[from] EvmEnvFactoryError),

    #[error("Chain Proof Client error: {0}")]
    ChainProofClient(#[from] chain_client::Error),

    #[error("Prover contract not deployed")]
    ProverContractNotDeployed,
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
    Engine(#[from] TravelCallExecutorError),

    #[error("Verification error: {0}")]
    Verification(#[from] VerificationError),

    #[error("Invalid input: {0}")]
    CreatingInput(String),

    #[error("Guest input verification error: {0}")]
    GuestInput(#[from] guest_input::Error),
}

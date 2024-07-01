use crate::db::proof::ProofDb;
use crate::into_input::into_input;
use crate::provider::EthersProviderError;
use crate::provider::{EthersProvider, Provider};
use alloy_primitives::Sealable;
use ethers_providers::Provider as OGEthersProvider;
use ethers_providers::{Http, ProviderError, RetryClient};
use guest_wrapper::GUEST_ELF;
use risc0_zkvm::{default_prover, ExecutorEnv};
use thiserror::Error;
use vlayer_engine::engine::{Engine, EngineError};
use vlayer_engine::ethereum::EthBlockHeader;
use vlayer_engine::io::GuestOutputError;
use vlayer_engine::io::{Call, GuestOutput, HostOutput, Input};

const MAX_RETRY: u32 = 3;
const INITIAL_BACKOFF: u64 = 500;

pub type EthersClient = OGEthersProvider<RetryClient<Http>>;

pub struct Host<P: Provider<Header = EthBlockHeader>> {
    db: ProofDb<P>,
    header: EthBlockHeader,
    config: HostConfig,
}

#[derive(Error, Debug)]
pub enum HostError {
    #[error("ExecutorEnv builder error")]
    ExecutorEnvBuilder(String),

    #[error("Invalid input")]
    CreatingInput(String),

    #[error("Engine error")]
    Engine(#[from] EngineError),

    #[error("Ethers provider error: {0}")]
    EthersProvider(#[from] EthersProviderError<ProviderError>),

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
}

pub struct HostConfig {
    url: String,
    start_chain_id: u64,
    start_block_number: u64,
}

impl HostConfig {
    pub fn new(url: &str, chain_id: u64, start_block_number: u64) -> Self {
        HostConfig {
            url: url.to_string(),
            start_chain_id: chain_id,
            start_block_number,
        }
    }
}

impl Host<EthersProvider<EthersClient>> {
    pub fn try_new(config: HostConfig) -> Result<Self, HostError> {
        let client = EthersClient::new_client(&config.url, MAX_RETRY, INITIAL_BACKOFF)?;

        let provider = EthersProvider::new(client);

        Host::try_new_with_provider(provider, config)
    }
}

impl<P: Provider<Header = EthBlockHeader>> Host<P> {
    pub fn try_new_with_provider(provider: P, config: HostConfig) -> Result<Self, HostError> {
        let start_block_number = config.start_block_number;
        let header = provider
            .get_block_header(start_block_number)
            .map_err(|err| HostError::Provider(err.to_string()))?
            .ok_or(HostError::BlockNotFound(start_block_number))?;

        let db = ProofDb::new(provider, start_block_number);

        Ok(Host { db, header, config })
    }

    pub fn run(mut self, call: Call) -> Result<HostOutput, HostError> {
        let engine = Engine::try_new(
            &mut self.db,
            self.header.clone(),
            self.config.start_chain_id,
        )?;
        let host_output = engine.call(&call)?;

        let evm_input = into_input(self.db, self.header.seal_slow())
            .map_err(|err| HostError::CreatingInput(err.to_string()))?;
        let input = Input { call, evm_input };
        let env = Self::build_executor_env(&input)?;

        let raw_guest_output = Self::prove(env, GUEST_ELF)?;
        let guest_output = GuestOutput::from_outputs(&host_output, &raw_guest_output)?;

        if guest_output.evm_call_result != host_output {
            return Err(HostError::HostGuestOutputMismatch(
                host_output,
                guest_output.evm_call_result,
            ));
        }

        Ok(HostOutput {
            guest_output,
            raw_abi: raw_guest_output,
        })
    }

    pub(crate) fn prove(env: ExecutorEnv, guest_elf: &[u8]) -> Result<Vec<u8>, HostError> {
        let prover = default_prover();
        prover
            .prove(env, guest_elf)
            .map(|p| p.receipt.journal.bytes)
            .map_err(|err| HostError::Prover(err.to_string()))
    }

    fn build_executor_env(input: &Input) -> Result<ExecutorEnv, HostError> {
        let env = ExecutorEnv::builder()
            .write(&input)
            .map_err(|err| HostError::ExecutorEnvBuilder(err.to_string()))?
            .build()
            .map_err(|err| HostError::ExecutorEnvBuilder(err.to_string()))?;
        Ok(env)
    }
}

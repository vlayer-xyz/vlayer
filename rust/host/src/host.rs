use alloy_primitives::Sealable;
use ethers_providers::Provider as OGEthersProvider;
use ethers_providers::{Http, ProviderError, RetryClient};
use guest_wrapper::GUEST_ELF;
use risc0_zkvm::{default_prover, ExecutorEnv};
use thiserror::Error;
use vlayer_engine::config::SEPOLIA_ID;
use vlayer_engine::ethereum::EthBlockHeader;
use vlayer_engine::guest::{Call, Input, Output};
use vlayer_engine::host::into_input;
use vlayer_engine::host::provider::EthersProviderError;
use vlayer_engine::{
    contract::engine,
    contract::engine::Engine,
    host::{
        db::ProofDb,
        provider::{EthersProvider, Provider},
    },
};

const MAX_RETRY: u32 = 3;
const INITIAL_BACKOFF: u64 = 500;

pub type EthersClient = OGEthersProvider<RetryClient<Http>>;

pub struct Host<P: Provider<Header = EthBlockHeader>> {
    db: ProofDb<P>,
    header: EthBlockHeader,
}

#[derive(Error, Debug)]
pub enum HostError {
    #[error("ExecutorEnv builder error")]
    ExecutorEnvBuilder(String),

    #[error("Invalid input")]
    CreatingInput(String),

    #[error("Engine error")]
    Engine(#[from] engine::EngineError),

    #[error("Ethers provider error: {0}")]
    EthersProvider(#[from] EthersProviderError<ProviderError>),

    #[error("Provider error: {0}")]
    Provider(#[from] ProviderError),

    #[error("Block not found: {0}")]
    BlockNotFound(u64),

    #[error("Error creating client: {0}")]
    NewClient(#[from] url::ParseError),

    #[error("Prover error: {0}")]
    Prover(String),
}

impl Host<EthersProvider<EthersClient>> {
    pub fn try_new(url: &str) -> Result<Self, HostError> {
        let client = EthersClient::new_client(url, MAX_RETRY, INITIAL_BACKOFF)?;
        let provider = EthersProvider::new(client);
        let block_number = provider.get_block_number()?;

        Host::try_new_with_provider(provider, block_number)
    }
}

impl<P: Provider<Header = EthBlockHeader, Error = EthersProviderError<ProviderError>>> Host<P> {
    pub fn try_new_with_provider(provider: P, block_number: u64) -> Result<Self, HostError> {
        let header = provider
            .get_block_header(block_number)?
            .ok_or(HostError::BlockNotFound(block_number))?;

        let db = ProofDb::new(provider, block_number);

        Ok(Host { db, header })
    }

    pub fn run(mut self, call: Call) -> Result<Output, HostError> {
        let _returns =
            Engine::try_new(&mut self.db, self.header.clone(), SEPOLIA_ID)?.call(&call)?;

        let input = Input {
            call,
            evm_input: into_input(self.db, self.header.seal_slow())
                .map_err(|err| HostError::CreatingInput(err.to_string()))?,
        };

        let env = ExecutorEnv::builder()
            .write(&input)
            .map_err(|err| HostError::ExecutorEnvBuilder(err.to_string()))?
            .build()
            .map_err(|err| HostError::ExecutorEnvBuilder(err.to_string()))?;

        Host::<P>::prove(env, GUEST_ELF)
    }

    pub(crate) fn prove(env: ExecutorEnv, guest_elf: &[u8]) -> Result<Output, HostError> {
        let prover = default_prover();
        prover
            .prove(env, guest_elf)
            .map(|p| p.receipt.journal.into())
            .map_err(|err| HostError::Prover(err.to_string()))
    }
}

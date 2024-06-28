use crate::db::proof::ProofDb;
use crate::into_input::into_input;
use crate::provider::EthersProviderError;
use crate::provider::{EthersProvider, Provider};
use alloy_primitives::Sealable;
use alloy_sol_types::SolValue;
use ethers_providers::Provider as OGEthersProvider;
use ethers_providers::{Http, ProviderError, RetryClient};
use guest_wrapper::GUEST_ELF;
use risc0_zkvm::{default_prover, ExecutorEnv};
use thiserror::Error;
use vlayer_engine::engine::{Engine, EngineError};
use vlayer_engine::ethereum::EthBlockHeader;
use vlayer_engine::io::{Call, GuestOutput, HostOutput, Input};
use vlayer_engine::SolCommitment;

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
}

pub struct HostConfig {
    url: String,
    chain_id: u64,
    block_number: u64,
}

impl HostConfig {
    pub fn new(url: &str, chain_id: u64, block_number: u64) -> Self {
        HostConfig {
            url: url.to_string(),
            chain_id,
            block_number,
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
        let block_number = config.block_number;
        let header = provider
            .get_block_header(block_number)
            .map_err(|err| HostError::Provider(err.to_string()))?
            .ok_or(HostError::BlockNotFound(block_number))?;

        let db = ProofDb::new(provider, block_number);

        Ok(Host { db, header, config })
    }

    pub fn run(mut self, call: Call) -> Result<HostOutput, HostError> {
        let preflight_returns =
            Engine::try_new(&mut self.db, self.header.clone(), self.config.chain_id)?
                .call(&call)?;

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

        let guest_returns = Host::<P>::prove(env, GUEST_ELF)?;

        let execution_commitment_len = guest_returns.len() - preflight_returns.len();

        let execution_commitment_abi_encoded = &guest_returns[..execution_commitment_len];
        let evm_call_result_abi_encoded = &guest_returns[execution_commitment_len..];

        assert_eq!(&preflight_returns, evm_call_result_abi_encoded);

        Ok(HostOutput {
            guest_output: GuestOutput {
                execution_commitment: SolCommitment::abi_decode(
                    execution_commitment_abi_encoded,
                    true,
                )
                .expect("Cannot decode execution commitment"),
                evm_call_result: evm_call_result_abi_encoded.to_vec(),
            },
            raw_abi: guest_returns,
        })
    }

    pub(crate) fn prove(env: ExecutorEnv, guest_elf: &[u8]) -> Result<Vec<u8>, HostError> {
        let prover = default_prover();
        prover
            .prove(env, guest_elf)
            .map(|p| p.receipt.journal.decode().expect("Cannot decode journal"))
            .map_err(|err| HostError::Prover(err.to_string()))
    }
}

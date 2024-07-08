use std::collections::HashMap;

use crate::db::proof::ProofDb;
use crate::into_input::into_input;
use crate::provider::EthersProviderError;
use crate::provider::{EthersProvider, Provider};
use alloy_primitives::{ChainId, Sealable};
use ethers_providers::Provider as OGEthersProvider;
use ethers_providers::{Http, ProviderError, RetryClient};
use guest_wrapper::GUEST_ELF;
use risc0_zkvm::{default_prover, ExecutorEnv};
use thiserror::Error;
use vlayer_engine::engine::{Engine, EngineError};
use vlayer_engine::ethereum::EthBlockHeader;
use vlayer_engine::evm::env::{EvmEnv, ExecutionLocation, MultiEvmEnv};
use vlayer_engine::evm::input::MultiEvmInput;
use vlayer_engine::io::GuestOutputError;
use vlayer_engine::io::{Call, GuestOutput, HostOutput, Input};

const MAX_RETRY: u32 = 3;
const INITIAL_BACKOFF: u64 = 500;

pub type EthersClient = OGEthersProvider<RetryClient<Http>>;

pub struct Host<P: Provider<Header = EthBlockHeader>> {
    start_execution_location: ExecutionLocation,
    envs: MultiEvmEnv<ProofDb<P>, EthBlockHeader>,
}

#[derive(Error, Debug)]
pub enum HostError {
    #[error("ExecutorEnv builder error")]
    ExecutorEnvBuilder(String),

    #[error("Invalid input")]
    CreatingInput(String),

    #[error("Engine error {0}")]
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

    #[error("No rpc url for chain: {0}")]
    NoRpcUrl(ChainId),
}

pub struct HostConfig {
    rpc_urls: HashMap<ChainId, String>,
    start_execution_location: ExecutionLocation,
}

impl HostConfig {
    pub fn new(url: &str, start_execution_location: ExecutionLocation) -> Self {
        let rpc_urls = [(start_execution_location.chain_id, url.to_string())]
            .into_iter()
            .collect();
        HostConfig {
            rpc_urls,
            start_execution_location,
        }
    }
}

impl Host<EthersProvider<EthersClient>> {
    pub fn try_new(config: HostConfig) -> Result<Self, HostError> {
        let chain_id = config.start_execution_location.chain_id;
        let url = config
            .rpc_urls
            .get(&chain_id)
            .ok_or(HostError::NoRpcUrl(chain_id))?;

        let client = EthersClient::new_client(&url, MAX_RETRY, INITIAL_BACKOFF)?;

        let provider = EthersProvider::new(client);

        Host::try_new_with_provider(provider, config)
    }
}

impl<P: Provider<Header = EthBlockHeader>> Host<P> {
    pub fn try_new_with_provider(provider: P, config: HostConfig) -> Result<Self, HostError> {
        let start_block_number = config.start_execution_location.block_number;
        let header = provider
            .get_block_header(start_block_number)
            .map_err(|err| HostError::Provider(err.to_string()))?
            .ok_or(HostError::BlockNotFound(start_block_number))?;

        let db = ProofDb::new(provider, start_block_number);
        let chain_spec = config.start_execution_location.chain_id.try_into()?;
        let env = EvmEnv::new(db, header.seal_slow()).with_chain_spec(&chain_spec)?;
        let envs = MultiEvmEnv::from_single(env, config.start_execution_location);

        Ok(Host {
            envs,
            start_execution_location: config.start_execution_location,
        })
    }

    pub fn run(mut self, call: Call) -> Result<HostOutput, HostError> {
        let env = self.envs.get_mut(&self.start_execution_location)?;
        let host_output = Engine::default().call(&call, env)?;

        let evm_input = into_input(&env.db, env.header.clone())
            .map_err(|err| HostError::CreatingInput(err.to_string()))?;
        let multi_evm_input = [(self.start_execution_location, evm_input)]
            .into_iter()
            .collect();
        let env = self.build_executor_env(multi_evm_input, call)?;

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

    fn build_executor_env(
        &self,
        multi_evm_input: MultiEvmInput<P::Header>,
        call: Call,
    ) -> Result<ExecutorEnv, HostError> {
        let input = Input {
            call,
            multi_evm_input,
            start_execution_location: self.start_execution_location,
        };
        let env = ExecutorEnv::builder()
            .write(&input)
            .map_err(|err| HostError::ExecutorEnvBuilder(err.to_string()))?
            .build()
            .map_err(|err| HostError::ExecutorEnvBuilder(err.to_string()))?;
        Ok(env)
    }
}

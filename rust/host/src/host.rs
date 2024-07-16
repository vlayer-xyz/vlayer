use crate::db::proof::ProofDb;
use crate::into_input::into_multi_input;
use crate::provider::factory::{EthersProviderFactory, ProviderFactory};
use crate::provider::{EthersProvider, Provider};
use crate::provider::{EthersProviderError, MultiProvider};
use crate::utils::get_or_insert_with_result;
use alloy_primitives::ChainId;
use ethers_providers::Provider as OGEthersProvider;
use ethers_providers::{Http, ProviderError, RetryClient};
use guest_wrapper::GUEST_ELF;
use risc0_ethereum_contracts::groth16::abi_encode;
use risc0_zkp::verify::VerificationError;
use risc0_zkvm::{default_prover, is_dev_mode, ExecutorEnv, ProverOpts};
use std::collections::HashMap;
use std::rc::Rc;
use thiserror::Error;
use vlayer_engine::engine::{Engine, EngineError};
use vlayer_engine::ethereum::EthBlockHeader;
use vlayer_engine::evm::env::{EvmEnv, ExecutionLocation, MultiEvmEnv};
use vlayer_engine::evm::input::MultiEvmInput;
use vlayer_engine::io::GuestOutputError;
use vlayer_engine::io::{Call, GuestOutput, HostOutput, Input};

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

    #[error("Verification error: {0}")]
    Verification(#[from] VerificationError),

    #[error("Abi encode error: {0}")]
    AbiEncode(String),

    #[error("No rpc cache for chain: {0}")]
    NoRpcCache(ChainId),
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
        let provider_factory = EthersProviderFactory::new(config.rpc_urls.clone());
        let provider = provider_factory.create(chain_id)?;

        Host::try_new_with_provider(Rc::new(provider), config)
    }
}

fn create_evm_env<P>(
    provider: Rc<P>,
    ExecutionLocation {
        block_number,
        chain_id,
    }: ExecutionLocation,
) -> Result<EvmEnv<ProofDb<P>, P::Header>, HostError>
where
    P: Provider,
{
    let header = provider
        .get_block_header(block_number)
        .map_err(|err| HostError::Provider(err.to_string()))?
        .ok_or(HostError::BlockNotFound(block_number))?;

    let db = ProofDb::new(provider, block_number);
    let chain_spec = chain_id.try_into()?;
    let mut env = EvmEnv::new(db, header);
    env.with_chain_spec(&chain_spec)?;
    Ok(env)
}

impl<P: Provider<Header = EthBlockHeader>> Host<P> {
    pub fn try_new_with_provider(provider: Rc<P>, config: HostConfig) -> Result<Self, HostError> {
        let env = create_evm_env(provider.clone(), config.start_execution_location)?;
        let envs = MultiEvmEnv::from([(config.start_execution_location, env)]);

        Ok(Host {
            envs,
            start_execution_location: config.start_execution_location,
        })
    }

    pub fn try_new_with_multi_provider(
        mut multi_provider: MultiProvider<P>,
        provider_factory: impl ProviderFactory<P>,
        config: HostConfig,
    ) -> Result<Self, HostError> {
        let chain_id = config.start_execution_location.chain_id;
        let provider = get_or_insert_with_result(&mut multi_provider, chain_id, || {
            Ok::<_, HostError>(Rc::new(provider_factory.create(chain_id)?))
        })?;

        Self::try_new_with_provider(provider, config)
    }

    pub fn run(mut self, call: Call) -> Result<HostOutput, HostError> {
        let env = self
            .envs
            .get_mut(&self.start_execution_location)
            .ok_or(HostError::Engine(EngineError::EvmEnvNotFound(
                self.start_execution_location,
            )))?;
        let host_output = Engine::default().call(&call, env)?;

        let multi_evm_input =
            into_multi_input(self.envs).map_err(|err| HostError::CreatingInput(err.to_string()))?;
        let env = Self::build_executor_env(self.start_execution_location, multi_evm_input, call)?;

        let (seal, raw_guest_output) = Self::prove(env, GUEST_ELF)?;
        let guest_output = GuestOutput::from_outputs(&host_output, &raw_guest_output)?;

        if guest_output.evm_call_result != host_output {
            return Err(HostError::HostGuestOutputMismatch(
                host_output,
                guest_output.evm_call_result,
            ));
        }

        Ok(HostOutput {
            guest_output,
            seal,
            raw_abi: raw_guest_output,
        })
    }

    pub(crate) fn prove(
        env: ExecutorEnv,
        guest_elf: &[u8],
    ) -> Result<(Vec<u8>, Vec<u8>), HostError> {
        let prover = default_prover();

        let receipt = prover
            .prove_with_opts(env, guest_elf, &ProverOpts::groth16())
            .map_err(|err| HostError::Prover(err.to_string()))?
            .receipt;

        let seal = if is_dev_mode() {
            Vec::new()
        } else {
            abi_encode(receipt.inner.groth16()?.seal.clone())
                .map_err(|err| HostError::AbiEncode(err.to_string()))?
        };

        Ok((seal, receipt.journal.bytes))
    }

    fn build_executor_env(
        start_execution_location: ExecutionLocation,
        multi_evm_input: MultiEvmInput<P::Header>,
        call: Call,
    ) -> Result<ExecutorEnv<'static>, HostError> {
        let input = Input {
            call,
            multi_evm_input,
            start_execution_location,
        };
        let env = ExecutorEnv::builder()
            .write(&input)
            .map_err(|err| HostError::ExecutorEnvBuilder(err.to_string()))?
            .build()
            .map_err(|err| HostError::ExecutorEnvBuilder(err.to_string()))?;
        Ok(env)
    }
}

use crate::evm_env::HostMultiEvmEnv;
use crate::into_input::into_multi_input;
use crate::multiprovider::{EthersMultiProvider, MultiProvider};
use crate::provider::EthersProviderError;
use crate::provider::{EthersProvider, Provider};
use std::collections::HashMap;

use alloy_primitives::ChainId;
use ethers_providers::Provider as OGEthersProvider;
use ethers_providers::{Http, ProviderError, RetryClient};
use guest_wrapper::GUEST_ELF;
use risc0_ethereum_contracts::groth16::abi_encode;
use risc0_zkp::verify::VerificationError;
use risc0_zkvm::{default_prover, is_dev_mode, ExecutorEnv, ProverOpts};
use thiserror::Error;
use vlayer_engine::engine::{Engine, EngineError};
use vlayer_engine::ethereum::EthBlockHeader;
use vlayer_engine::evm::env::{ExecutionLocation, MultiEvmEnv};
use vlayer_engine::evm::input::MultiEvmInput;
use vlayer_engine::io::GuestOutputError;
use vlayer_engine::io::{Call, GuestOutput, HostOutput, Input};

pub type EthersClient = OGEthersProvider<RetryClient<Http>>;

pub struct Host<P: Provider<Header = EthBlockHeader>, M: MultiProvider<P>> {
    start_execution_location: ExecutionLocation,
    envs: HostMultiEvmEnv<P, M>,
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

impl Host<EthersProvider<EthersClient>, EthersMultiProvider> {
    pub fn try_new(config: HostConfig) -> Result<Self, HostError> {
        let multi_provider = EthersMultiProvider::new(config.rpc_urls.clone());

        Host::try_new_with_multi_provider(multi_provider, config)
    }
}

impl<P: Provider<Header = EthBlockHeader>, M: MultiProvider<P>> Host<P, M> {
    pub fn try_new_with_multi_provider(
        multi_provider: M,
        config: HostConfig,
    ) -> Result<Self, HostError> {
        let envs = HostMultiEvmEnv::new(multi_provider);

        Ok(Host {
            envs,
            start_execution_location: config.start_execution_location,
        })
    }

    pub fn run(mut self, call: Call) -> Result<HostOutput, HostError> {
        self.envs.ensure_vm_exists(self.start_execution_location)?;
        let env = self.envs.get_mut(&self.start_execution_location)?;
        let host_output = Engine::default().call(&call, env)?;

        let multi_evm_input =
            into_multi_input(self.envs).map_err(|err| HostError::CreatingInput(err.to_string()))?;
        let env = build_executor_env(self.start_execution_location, multi_evm_input, call)?;

        let (seal, raw_guest_output) = prove(env, GUEST_ELF)?;
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
}

pub(crate) fn prove(env: ExecutorEnv, guest_elf: &[u8]) -> Result<(Vec<u8>, Vec<u8>), HostError> {
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
    multi_evm_input: MultiEvmInput<EthBlockHeader>,
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

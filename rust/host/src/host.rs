use crate::db::proof::ProofDb;
use crate::evm_env::factory::EvmEnvFactory;
use crate::into_input::into_multi_input;
use crate::provider::factory::{EthersProviderFactory, ProviderFactory};
use crate::provider::multi::CachedMultiProvider;
use crate::provider::{EthersClient, EthersProvider, Provider};
use crate::utils::get_mut_or_insert_with_result;
use config::HostConfig;
use error::HostError;
use guest_wrapper::GUEST_ELF;
use risc0_ethereum_contracts::groth16::abi_encode;
use risc0_zkvm::{default_prover, is_dev_mode, ExecutorEnv, ProverOpts};
use std::collections::HashMap;
use vlayer_engine::engine::Engine;
use vlayer_engine::ethereum::EthBlockHeader;
use vlayer_engine::evm::env::{EvmEnv, ExecutionLocation, MultiEvmEnv};
use vlayer_engine::evm::input::MultiEvmInput;
use vlayer_engine::io::{Call, GuestOutput, HostOutput, Input};

pub mod config;
pub mod error;

pub struct Host<P: Provider<Header = EthBlockHeader>> {
    start_execution_location: ExecutionLocation,
    envs: MultiEvmEnv<ProofDb<P>, EthBlockHeader>,
    env_factory: EvmEnvFactory<P>,
}

impl Host<EthersProvider<EthersClient>> {
    pub fn try_new(config: HostConfig) -> Result<Self, HostError> {
        let provider_factory = EthersProviderFactory::new(config.rpc_urls.clone());
        Host::try_new_with_provider_factory(provider_factory, config)
    }
}

impl<P: Provider<Header = EthBlockHeader>> Host<P> {
    pub fn try_new_with_provider_factory(
        provider_factory: impl ProviderFactory<P> + 'static,
        config: HostConfig,
    ) -> Result<Self, HostError> {
        let providers = CachedMultiProvider::new(provider_factory);
        let env_factory = EvmEnvFactory::new(providers);
        let envs = HashMap::new();

        Ok(Host {
            envs,
            env_factory,
            start_execution_location: config.start_execution_location,
        })
    }

    pub fn run(mut self, call: Call) -> Result<HostOutput, HostError> {
        let env = self.get_mut_env(self.start_execution_location)?;
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

    fn get_mut_env(
        &mut self,
        location: ExecutionLocation,
    ) -> Result<&mut EvmEnv<ProofDb<P>, P::Header>, HostError> {
        get_mut_or_insert_with_result(&mut self.envs, location, || {
            self.env_factory.create(location)
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

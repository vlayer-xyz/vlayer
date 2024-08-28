use crate::db::proof::ProofDb;
use crate::encodable_receipt::EncodableReceipt;
use crate::evm_env::factory::HostEvmEnvFactory;
use crate::into_input::into_multi_input;
use crate::provider::factory::{EthersProviderFactory, ProviderFactory};
use crate::provider::multi::CachedMultiProvider;
use crate::provider::{BlockingProvider, EthersClient, EthersProvider};
use alloy_primitives::ChainId;
use alloy_sol_types::SolValue;
use call_engine::engine::Engine;
use call_engine::evm::env::{cached::CachedEvmEnv, location::ExecutionLocation};
use call_engine::evm::input::MultiEvmInput;
use call_engine::io::{Call, GuestOutput, HostOutput, Input};
use call_engine::Seal;
use call_guest_wrapper::RISC0_CALL_GUEST_ELF;
use config::HostConfig;
use error::HostError;
use ethers_core::types::BlockNumber;
use risc0_zkvm::{default_prover, ExecutorEnv, ProverOpts};

pub mod config;
pub mod error;

pub struct Host<P: BlockingProvider> {
    start_execution_location: ExecutionLocation,
    envs: CachedEvmEnv<ProofDb<P>>,
}

impl Host<EthersProvider<EthersClient>> {
    pub fn try_new(config: HostConfig) -> Result<Self, HostError> {
        let provider_factory = EthersProviderFactory::new(config.rpc_urls.clone());
        Host::try_new_with_provider_factory(provider_factory, config)
    }
}

fn get_block_number(
    providers: &CachedMultiProvider<impl BlockingProvider>,
    chain_id: ChainId,
) -> Result<ChainId, HostError> {
    let provider = providers.get(chain_id)?;
    let block_header = provider
        .get_block_header(BlockNumber::Latest)
        .map_err(|e| HostError::Provider(format!("Error fetching block header: {:?}", e)))?
        .ok_or_else(|| HostError::Provider(String::from("Block header not found")))?;
    Ok((*block_header).number())
}

impl<P> Host<P>
where
    P: BlockingProvider + 'static,
{
    pub fn try_new_with_provider_factory(
        provider_factory: impl ProviderFactory<P> + 'static,
        config: HostConfig,
    ) -> Result<Self, HostError> {
        let providers = CachedMultiProvider::new(provider_factory);
        let block_number = get_block_number(&providers, config.start_chain_id)?;
        let envs = CachedEvmEnv::from_factory(HostEvmEnvFactory::new(providers));
        let start_execution_location = ExecutionLocation::new(block_number, config.start_chain_id);

        Ok(Host {
            envs,
            start_execution_location,
        })
    }

    pub fn try_new_with_provider_factory_and_block_number(
        provider_factory: impl ProviderFactory<P> + 'static,
        config: HostConfig,
        block_number: u64,
    ) -> Result<Self, HostError> {
        let providers = CachedMultiProvider::new(provider_factory);
        let envs = CachedEvmEnv::from_factory(HostEvmEnvFactory::new(providers));
        let start_execution_location = ExecutionLocation::new(block_number, config.start_chain_id);

        Ok(Host {
            envs,
            start_execution_location,
        })
    }

    pub fn run(self, call: Call) -> Result<HostOutput, HostError> {
        let host_output = Engine::new(&self.envs).call(&call, self.start_execution_location)?;

        let multi_evm_input =
            into_multi_input(self.envs).map_err(|err| HostError::CreatingInput(err.to_string()))?;
        let env = Self::build_executor_env(self.start_execution_location, multi_evm_input, call)?;
        let (seal, raw_guest_output) = Self::prove(env, RISC0_CALL_GUEST_ELF)?;
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

    fn prove(env: ExecutorEnv, guest_elf: &[u8]) -> Result<(Vec<u8>, Vec<u8>), HostError> {
        let prover = default_prover();

        let result = prover
            .prove_with_opts(env, guest_elf, &ProverOpts::groth16())
            .map_err(|err| HostError::Prover(err.to_string()))?;

        let seal: Seal = EncodableReceipt::from(result.receipt.clone()).try_into()?;

        Ok((seal.abi_encode(), result.receipt.journal.bytes))
    }

    fn build_executor_env(
        start_execution_location: ExecutionLocation,
        multi_evm_input: MultiEvmInput,
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

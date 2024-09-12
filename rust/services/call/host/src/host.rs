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
use call_engine::io::{Call, GuestOutput, HostOutput, Input};
use call_engine::Seal;
use call_guest_wrapper::RISC0_CALL_GUEST_ELF;
use config::HostConfig;
use error::HostError;
use ethers_core::types::BlockNumber;
use host_utils::Prover;
use risc0_zkvm::ExecutorEnv;
use serde::Serialize;

pub mod config;
pub mod error;

pub struct Host<P: BlockingProvider> {
    start_execution_location: ExecutionLocation,
    envs: CachedEvmEnv<ProofDb<P>>,
    prover: Prover,
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
        let prover = Prover::new(config.proof_mode);

        Ok(Host {
            envs,
            start_execution_location,
            prover,
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
        let prover = Prover::new(config.proof_mode);

        Ok(Host {
            envs,
            start_execution_location,
            prover,
        })
    }

    pub fn run(self, call: Call) -> Result<HostOutput, HostError> {
        let host_output = Engine::new(&self.envs).call(&call, self.start_execution_location)?;

        let multi_evm_input =
            into_multi_input(self.envs).map_err(|err| HostError::CreatingInput(err.to_string()))?;
        let input = Input {
            call,
            multi_evm_input,
            start_execution_location: self.start_execution_location,
        };

        let env = build_executor_env(input)
            .map_err(|err| HostError::ExecutorEnvBuilder(err.to_string()))?;
        let (seal, raw_guest_output) = prove(&self.prover, env, RISC0_CALL_GUEST_ELF)?;

        let proof_len = raw_guest_output.len();
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
            proof_len,
        })
    }
}

fn prove(
    prover: &Prover,
    env: ExecutorEnv,
    guest_elf: &[u8],
) -> Result<(Vec<u8>, Vec<u8>), HostError> {
    let result = prover
        .prove(env, guest_elf)
        .map_err(|err| HostError::Prover(err.to_string()))?;

    let seal: Seal = EncodableReceipt::from(result.receipt.clone()).try_into()?;

    Ok((seal.abi_encode(), result.receipt.journal.bytes))
}

fn build_executor_env(input: impl Serialize) -> anyhow::Result<ExecutorEnv<'static>> {
    ExecutorEnv::builder().write(&input)?.build()
}

#[cfg(test)]
mod test {
    use super::*;

    use chain::TEST_CHAIN_ID_1;
    use host_utils::ProofMode;

    #[test]
    fn host_prove_invalid_guest_elf() {
        let prover = Prover::default();
        let env = ExecutorEnv::default();
        let invalid_guest_elf = &[];
        let res = prove(&prover, env, invalid_guest_elf);

        assert!(matches!(
            res.map(|_| ()).unwrap_err(),
            HostError::Prover(ref msg) if msg == "Elf parse error: Could not read bytes in range [0x0, 0x10)"
        ));
    }

    #[test]
    fn host_prove_invalid_input() {
        let prover = Prover::default();
        let env = ExecutorEnv::default();
        let res = prove(&prover, env, RISC0_CALL_GUEST_ELF);

        assert!(matches!(
            res.map(|_| ()).unwrap_err(),
            HostError::Prover(ref msg) if msg == "Guest panicked: called `Result::unwrap()` on an `Err` value: DeserializeUnexpectedEnd"
        ));
    }

    #[test]
    fn try_new_invalid_rpc_url() -> anyhow::Result<()> {
        let rpc_urls = [(TEST_CHAIN_ID_1, "http://localhost:123/".to_string())]
            .into_iter()
            .collect();
        let config = HostConfig {
            rpc_urls,
            start_chain_id: TEST_CHAIN_ID_1,
            proof_mode: ProofMode::Fake,
        };
        let res = Host::try_new(config);

        assert!(matches!(
            res.map(|_| ()).unwrap_err(),
            HostError::Provider(ref msg) if msg.to_string().contains(
                "Error fetching block header"
            )
        ));

        Ok(())
    }
}

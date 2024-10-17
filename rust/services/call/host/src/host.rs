use std::{
    any::Any,
    panic::{self},
};

use alloy_primitives::ChainId;
use alloy_sol_types::SolValue;
use call_engine::{
    engine::{Engine, EngineError, SuccessfulExecutionResult},
    evm::env::{cached::CachedEvmEnv, location::ExecutionLocation},
    io::{Call, GuestOutput, HostOutput, Input},
    Seal,
};
use call_guest_wrapper::RISC0_CALL_GUEST_ELF;
use chain_client::ChainProofClient;
use config::HostConfig;
use error::HostError;
use ethers_core::types::BlockNumber;
use host_utils::Prover;
use provider::{BlockingProvider, CachedMultiProvider, EthProvider, EthersProviderFactory};
use risc0_zkvm::ExecutorEnv;
use serde::Serialize;
use tracing::info;

use crate::{
    db::proof::ProofDb, encodable_receipt::EncodableReceipt, evm_env::factory::HostEvmEnvFactory,
    into_input::into_multi_input,
};

pub mod config;
pub mod error;

pub struct Host<P: BlockingProvider> {
    start_execution_location: ExecutionLocation,
    envs: CachedEvmEnv<ProofDb<P>>,
    prover: Prover,
    _chain_proof_client: ChainProofClient,
    max_request_size: usize,
}

impl Host<EthProvider> {
    pub fn try_new(config: &HostConfig) -> Result<Self, HostError> {
        let provider_factory = EthersProviderFactory::new(config.rpc_urls.clone());
        let providers = CachedMultiProvider::new(provider_factory);
        let block_number = get_block_number(&providers, config.start_chain_id)?;
        let chain_proof_client = ChainProofClient::new(config.chain_proof_url.clone());

        Host::try_new_with_components(providers, block_number, chain_proof_client, config)
    }
}

pub fn get_block_number(
    providers: &CachedMultiProvider<impl BlockingProvider>,
    chain_id: ChainId,
) -> Result<ChainId, HostError> {
    let provider = providers.get(chain_id)?;

    let block_number = provider
        .get_block_header(BlockNumber::Latest)
        .map_err(|e| HostError::Provider(format!("Error fetching block header: {:?}", e)))?
        .ok_or_else(|| HostError::Provider(String::from("Block header not found")))
        .map(|block_header| (*block_header).number())?;

    Ok(block_number)
}

impl<P> Host<P>
where
    P: BlockingProvider + 'static,
{
    pub fn try_new_with_components(
        providers: CachedMultiProvider<P>,
        block_number: u64,
        chain_proof_client: ChainProofClient,
        config: &HostConfig,
    ) -> Result<Self, HostError> {
        let envs = CachedEvmEnv::from_factory(HostEvmEnvFactory::new(providers));
        let start_execution_location = (block_number, config.start_chain_id).into();
        let prover = Prover::new(config.proof_mode);

        Ok(Host {
            envs,
            start_execution_location,
            prover,
            _chain_proof_client: chain_proof_client,
            max_request_size: config.max_request_size,
        })
    }

    #[allow(clippy::unused_async)]
    pub async fn run(self, call: Call) -> Result<HostOutput, HostError> {
        self.validate_call_size(&call)?;

        let SuccessfulExecutionResult {
            output: host_output,
            gas_used,
        } = panic::catch_unwind(|| {
            Engine::new(&self.envs).call(&call, self.start_execution_location)
        })
        .map_err(wrap_engine_panic)??;

        info!("Gas used in preflight: {}", gas_used);

        let multi_evm_input =
            into_multi_input(self.envs).map_err(|err| HostError::CreatingInput(err.to_string()))?;
        // let chain_proofs: std::collections::HashMap<u64, chain_types::ChainProof> = self
        //     .chain_proof_client
        //     .get_chain_proofs(multi_evm_input.group_blocks_by_chain())
        //     .await?;
        let chain_proofs: std::collections::HashMap<u64, chain_types::ChainProof> =
            std::collections::HashMap::new();
        let input = Input {
            call,
            multi_evm_input,
            start_execution_location: self.start_execution_location,
            chain_proofs,
        };

        let env = build_executor_env(input)
            .map_err(|err| HostError::ExecutorEnvBuilder(err.to_string()))?;
        let (seal, raw_guest_output) = provably_execute(&self.prover, env, RISC0_CALL_GUEST_ELF)?;

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

    fn validate_call_size(&self, call: &Call) -> Result<(), HostError> {
        if call.data.len() > self.max_request_size {
            return Err(HostError::InputTooLargeError(call.data.len()));
        }

        Ok(())
    }
}

fn wrap_engine_panic(err: Box<dyn Any + Send>) -> EngineError {
    let panic_msg = err
        .downcast::<String>()
        .map(|x| *x)
        .unwrap_or("Panic occurred".to_string());
    EngineError::Panic(panic_msg)
}

fn provably_execute(
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
    use std::collections::HashMap;

    use chain::TEST_CHAIN_ID;
    use host_utils::ProofMode;

    use super::*;

    fn test_rpc_urls() -> HashMap<ChainId, String> {
        [(TEST_CHAIN_ID, "http://localhost:123/".to_string())]
            .into_iter()
            .collect()
    }

    #[test]
    fn host_provably_execute_invalid_guest_elf() {
        let prover = Prover::default();
        let env = ExecutorEnv::default();
        let invalid_guest_elf = &[];
        let res = provably_execute(&prover, env, invalid_guest_elf);

        assert!(matches!(
            res.map(|_| ()).unwrap_err(),
            HostError::Prover(ref msg) if msg == "Elf parse error: Could not read bytes in range [0x0, 0x10)"
        ));
    }

    #[test]
    fn host_provably_execute_invalid_input() {
        let prover = Prover::default();
        let env = ExecutorEnv::default();
        let res = provably_execute(&prover, env, RISC0_CALL_GUEST_ELF);

        assert!(matches!(
            res.map(|_| ()).unwrap_err(),
            HostError::Prover(ref msg) if msg == "Guest panicked: called `Result::unwrap()` on an `Err` value: DeserializeUnexpectedEnd"
        ));
    }

    #[tokio::test(flavor = "multi_thread")]
    async fn host_does_not_accept_calls_longer_than_limit() -> anyhow::Result<()> {
        let config = HostConfig {
            rpc_urls: test_rpc_urls(),
            start_chain_id: TEST_CHAIN_ID,
            proof_mode: ProofMode::Fake,
            ..HostConfig::default()
        };
        let host = Host::try_new_with_components(
            CachedMultiProvider::new(EthersProviderFactory::new(test_rpc_urls())),
            0,
            ChainProofClient::new(""),
            &config,
        )?;
        let call = Call {
            to: Default::default(),
            data: vec![0; config.max_request_size + 1],
        };
        assert_eq!(
            host.run(call).await.unwrap_err().to_string(),
            format!("Call input too large: {} bytes", config.max_request_size + 1)
        );

        Ok(())
    }

    #[tokio::test(flavor = "multi_thread")]
    async fn try_new_invalid_rpc_url() -> anyhow::Result<()> {
        let config = HostConfig {
            rpc_urls: test_rpc_urls(),
            start_chain_id: TEST_CHAIN_ID,
            proof_mode: ProofMode::Fake,
            ..HostConfig::default()
        };
        let res = Host::try_new(&config);

        assert!(matches!(
            res.map(|_| ()).unwrap_err(),
            HostError::Provider(ref msg) if msg.to_string().contains(
                "Error fetching block header"
            )
        ));

        Ok(())
    }
}

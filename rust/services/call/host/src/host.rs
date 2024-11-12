use std::{
    any::Any,
    panic::{self},
};

use alloy_primitives::ChainId;
use alloy_sol_types::SolValue;
use call_engine::{
    evm::env::{cached::CachedEvmEnv, location::ExecutionLocation},
    travel_call_executor::{
        Error as TravelCallExecutorError, SuccessfulExecutionResult, TravelCallExecutor,
    },
    Call, GuestOutput, HostOutput, Input, Seal,
};
use call_guest_wrapper::RISC0_CALL_GUEST_ID;
use chain_client::{
    Client as ChainProofClient, RecordingClient as RecordingRpcClient,
    RpcClient as RpcChainProofClient,
};
use config::HostConfig;
use error::HostError;
use ethers_core::types::BlockNumber;
use prover::Prover;
use provider::{CachedMultiProvider, EthersProviderFactory};
use risc0_zkvm::sha::Digest;
use tracing::info;

use crate::{
    db::proof::ProofDb, encodable_receipt::EncodableReceipt, evm_env::factory::HostEvmEnvFactory,
    into_input::into_multi_input,
};

pub mod config;
pub mod error;
mod prover;

pub struct Host {
    start_execution_location: ExecutionLocation,
    envs: CachedEvmEnv<ProofDb>,
    prover: Prover,
    chain_proof_client: RecordingRpcClient,
    max_calldata_size: usize,
}

impl Host {
    pub fn try_new(config: &HostConfig) -> Result<Self, HostError> {
        let provider_factory = EthersProviderFactory::new(config.rpc_urls.clone());
        let providers = CachedMultiProvider::new(provider_factory);
        let block_number = get_block_number(&providers, config.start_chain_id)?;
        let chain_proof_client = RpcChainProofClient::new(&config.chain_proof_url);

        Host::try_new_with_components(providers, block_number, chain_proof_client, config)
    }
}

pub fn get_block_number(
    providers: &CachedMultiProvider,
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

impl Host {
    pub fn try_new_with_components(
        providers: CachedMultiProvider,
        block_number: u64,
        chain_proof_client: impl ChainProofClient,
        config: &HostConfig,
    ) -> Result<Self, HostError> {
        validate_guest_image_id(config.call_guest_id)?;

        let envs = CachedEvmEnv::from_factory(HostEvmEnvFactory::new(providers));
        let start_execution_location = (block_number, config.start_chain_id).into();
        let prover = Prover::new(config.proof_mode);
        let chain_proof_client = RecordingRpcClient::new(chain_proof_client);

        Ok(Host {
            envs,
            start_execution_location,
            prover,
            chain_proof_client,
            max_calldata_size: config.max_calldata_size,
        })
    }

    pub async fn main(self, call: Call) -> Result<HostOutput, HostError> {
        self.validate_calldata_size(&call)?;

        let SuccessfulExecutionResult {
            output: host_output,
            gas_used,
        } = panic::catch_unwind(|| {
            TravelCallExecutor::new(&self.envs).call(&call, self.start_execution_location)
        })
        .map_err(wrap_engine_panic)??;

        info!("Gas used in preflight: {}", gas_used);

        let multi_evm_input =
            into_multi_input(self.envs).map_err(|err| HostError::CreatingInput(err.to_string()))?;
        _ = self
            .chain_proof_client
            .get_chain_proofs(multi_evm_input.block_nums_by_chain())
            .await?;
        let chain_proofs = self.chain_proof_client.into_cache();
        let input = Input {
            multi_evm_input,
            start_execution_location: self.start_execution_location,
            chain_proofs,
            call,
        };

        let (seal, raw_guest_output) = provably_execute(&self.prover, &input)?;

        let proof_len = raw_guest_output.len();
        let guest_output = GuestOutput::from_outputs(&host_output, &raw_guest_output)?;

        if guest_output.evm_call_result != host_output {
            return Err(HostError::HostGuestOutputMismatch(
                host_output,
                guest_output.evm_call_result,
            ));
        }

        let call_guest_id: Digest = RISC0_CALL_GUEST_ID.into();

        Ok(HostOutput {
            guest_output,
            seal,
            raw_abi: raw_guest_output,
            proof_len,
            call_guest_id: call_guest_id.into(),
        })
    }

    fn validate_calldata_size(&self, call: &Call) -> Result<(), HostError> {
        if call.data.len() > self.max_calldata_size {
            return Err(HostError::CalldataTooLargeError(call.data.len()));
        }

        Ok(())
    }
}

fn wrap_engine_panic(err: Box<dyn Any + Send>) -> TravelCallExecutorError {
    let panic_msg = err
        .downcast::<String>()
        .map(|x| *x)
        .unwrap_or("Panic occurred".to_string());
    TravelCallExecutorError::Panic(panic_msg)
}

fn provably_execute(prover: &Prover, input: &Input) -> Result<(Vec<u8>, Vec<u8>), HostError> {
    let receipt = prover.prove(input)?;

    let seal: Seal = EncodableReceipt::from(receipt.clone()).try_into()?;

    Ok((seal.abi_encode(), receipt.journal.bytes))
}

fn validate_guest_image_id(image_id: Digest) -> Result<(), HostError> {
    if image_id != RISC0_CALL_GUEST_ID.into() {
        return Err(HostError::UnsupportedCallGuestId(RISC0_CALL_GUEST_ID.into(), image_id));
    }

    Ok(())
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
            RpcChainProofClient::new(""),
            &config,
        )?;
        let call = Call {
            to: Default::default(),
            data: vec![0; config.max_calldata_size + 1],
        };
        assert_eq!(
            host.main(call).await.unwrap_err().to_string(),
            format!("Calldata too large: {} bytes", config.max_calldata_size + 1)
        );

        Ok(())
    }

    mod try_new {
        use super::*;

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

        #[test]
        fn fails_with_mismatched_image_id() {
            let config = HostConfig {
                call_guest_id: Default::default(),
                ..HostConfig::default()
            };

            let res = Host::try_new_with_components(
                CachedMultiProvider::new(EthersProviderFactory::new(test_rpc_urls())),
                0,
                RpcChainProofClient::new(""),
                &config,
            );

            assert!(matches!(
                res.map(|_| ()).unwrap_err(),
                HostError::UnsupportedCallGuestId(ref expected, ref received) if expected == &(RISC0_CALL_GUEST_ID.into()) && received == &Default::default()
            ));
        }
    }
}

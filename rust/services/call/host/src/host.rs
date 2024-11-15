use std::{
    any::Any,
    panic::{self},
};

use alloy_primitives::{BlockNumber, ChainId};
use alloy_sol_types::SolValue;
use call_engine::{
    evm::env::{cached::CachedEvmEnv, location::ExecutionLocation},
    travel_call_executor::{
        Error as TravelCallExecutorError, SuccessfulExecutionResult, TravelCallExecutor,
    },
    verifier::{
        chain_proof,
        guest_input::{self, Verifier},
        zk_proof,
    },
    Call, GuestOutput, HostOutput, Input, Seal,
};
use common::GuestElf;
use config::HostConfig;
use error::HostError;
use ethers_core::types::BlockNumber as BlockTag;
use prover::Prover;
use provider::{CachedMultiProvider, EthersProviderFactory, EvmBlockHeader};
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
    verifier: guest_input::ZkVerifier<chain_client::RecordingClient, chain_proof::ZkVerifier>,
    max_calldata_size: usize,
    verify_chain_proofs: bool,
    guest_elf: GuestElf,
}

impl Host {
    pub fn try_new(config: HostConfig) -> Result<Self, HostError> {
        let provider_factory = EthersProviderFactory::new(config.rpc_urls.clone());
        let providers = CachedMultiProvider::new(provider_factory);
        let block_number = get_latest_block_number(&providers, config.start_chain_id)?;
        let chain_client = chain_client::RpcClient::new(&config.chain_proof_url);
        Host::try_new_with_components(providers, block_number, chain_client, config)
    }
}

pub fn get_latest_block_number(
    providers: &CachedMultiProvider,
    chain_id: ChainId,
) -> Result<BlockNumber, HostError> {
    get_block_header(providers, chain_id, BlockTag::Latest).map(|header| header.number())
}

pub fn get_block_header(
    providers: &CachedMultiProvider,
    chain_id: ChainId,
    block_num: BlockTag,
) -> Result<Box<dyn EvmBlockHeader>, HostError> {
    let provider = providers.get(chain_id)?;

    let block_header = provider
        .get_block_header(block_num)
        .map_err(|e| HostError::Provider(format!("Error fetching block header: {:?}", e)))?
        .ok_or_else(|| HostError::Provider(String::from("Block header not found")))?;

    Ok(block_header)
}

impl Host {
    pub fn try_new_with_components(
        providers: CachedMultiProvider,
        block_number: u64,
        chain_client: impl chain_client::Client,
        config: HostConfig,
    ) -> Result<Self, HostError> {
        let envs = CachedEvmEnv::from_factory(HostEvmEnvFactory::new(providers));
        let start_execution_location = (block_number, config.start_chain_id).into();
        let prover = Prover::new(config.proof_mode, &config.call_guest_elf);
        let chain_client = chain_client::RecordingClient::new(chain_client);
        let chain_proof_verifier =
            chain_proof::ZkVerifier::new(config.chain_guest_elf.id, zk_proof::HostVerifier);
        let verifier = guest_input::ZkVerifier::new(chain_client, chain_proof_verifier);

        Ok(Host {
            envs,
            start_execution_location,
            prover,
            verifier,
            max_calldata_size: config.max_calldata_size,
            verify_chain_proofs: config.verify_chain_proofs,
            guest_elf: config.call_guest_elf,
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

        let chain_proofs = if self.verify_chain_proofs {
            self.verifier.verify(&multi_evm_input).await?;
            let (chain_proof_client, _) = self.verifier.into_parts();
            Some(chain_proof_client.into_cache())
        } else {
            None
        };

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

        Ok(HostOutput {
            guest_output,
            seal,
            raw_abi: raw_guest_output,
            proof_len,
            call_guest_id: self.guest_elf.id.into(),
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
        let max_call_data_size = config.max_calldata_size;
        let host = Host::try_new_with_components(
            CachedMultiProvider::new(EthersProviderFactory::new(test_rpc_urls())),
            0,
            chain_client::RpcClient::new(""),
            config,
        )?;
        let call = Call {
            to: Default::default(),
            data: vec![0; max_call_data_size + 1],
        };

        assert_eq!(
            host.main(call).await.unwrap_err().to_string(),
            format!("Calldata too large: {} bytes", max_call_data_size + 1)
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
}

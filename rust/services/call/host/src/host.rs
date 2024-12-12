use std::{
    any::Any,
    panic::{self},
};

use alloy_primitives::{BlockNumber, ChainId};
use alloy_sol_types::SolValue;
use bytes::Bytes;
use call_engine::{
    evm::{
        env::{cached::CachedEvmEnv, location::ExecutionLocation},
        execution_result::SuccessfulExecutionResult,
    },
    travel_call_executor::{Error as TravelCallExecutorError, TravelCallExecutor},
    verifier::{
        chain_proof,
        guest_input::{self, Verifier},
        zk_proof,
    },
    Call, CallGuestId, GuestOutput, HostOutput, Input, Seal,
};
use chain_client::Client;
use common::GuestElf;
pub use config::{Config, DEFAULT_MAX_CALLDATA_SIZE};
use derive_new::new;
pub use error::Error;
use ethers_core::types::BlockNumber as BlockTag;
use mock_chain_server::ChainProofServerMock;
use prover::Prover;
use provider::{CachedMultiProvider, EthersProviderFactory, EvmBlockHeader};
use seal::EncodableReceipt;
use tracing::{info, warn};

use crate::{evm_env::factory::HostEvmEnvFactory, into_input::into_multi_input, HostDb};

mod config;
mod error;
mod prover;
#[cfg(test)]
mod tests;

pub struct Host {
    start_execution_location: ExecutionLocation,
    envs: CachedEvmEnv<HostDb>,
    prover: Prover,
    verifier: guest_input::ZkVerifier<chain_client::RecordingClient, chain_proof::ZkVerifier>,
    max_calldata_size: usize,
    verify_chain_proofs: bool,
    guest_elf: GuestElf,
}

async fn mock_chain_client(
    providers: &CachedMultiProvider,
    chain_id: ChainId,
) -> Result<(BlockNumber, Box<dyn chain_client::Client>), Error> {
    let latest_block = providers
        .get(chain_id)?
        .get_block_header(BlockTag::Latest)?
        .ok_or_else(|| Error::Provider("latest block not found".to_string()))?;
    let block_number = latest_block.number();
    let mut chain_proof_server = ChainProofServerMock::start().await;
    chain_proof_server
        .mock_single_block(chain_id, latest_block)
        .await;
    let chain_client = Box::new(chain_proof_server.into_client());
    Ok((block_number, chain_client))
}

impl Host {
    pub async fn try_new(config: Config) -> Result<Self, Error> {
        let provider_factory = EthersProviderFactory::new(config.rpc_urls.clone());
        let providers = CachedMultiProvider::from_factory(provider_factory);

        let (block_number, chain_client) = match config.chain_proof_url.as_ref() {
            Some(url) => {
                let chain_client = chain_client::RpcClient::new(url);
                // If a chain server is running, use latest block **for which we have chain proof**
                // as the settlement block. For newer block getting chain proof would fail.
                let block_number = chain_client
                    .get_sync_status(config.start_chain_id)
                    .await?
                    .last_block;
                (block_number, Box::new(chain_client) as Box<dyn Client>)
            }
            None => {
                warn!("Chain proof sever URL not provided. Running with mock server");
                mock_chain_client(&providers, config.start_chain_id).await?
            }
        };

        Host::try_new_with_components(providers, block_number, chain_client, config)
    }

    pub fn prover(&self) -> Prover {
        self.prover.clone()
    }

    pub const fn start_execution_location(&self) -> ExecutionLocation {
        self.start_execution_location
    }

    pub fn call_guest_id(&self) -> CallGuestId {
        self.guest_elf.id.into()
    }
}

pub fn get_latest_block_number(
    providers: &CachedMultiProvider,
    chain_id: ChainId,
) -> Result<BlockNumber, Error> {
    get_block_header(providers, chain_id, BlockTag::Latest).map(|header| header.number())
}

pub fn get_block_header(
    providers: &CachedMultiProvider,
    chain_id: ChainId,
    block_num: BlockTag,
) -> Result<Box<dyn EvmBlockHeader>, Error> {
    let provider = providers.get(chain_id)?;

    let block_header = provider
        .get_block_header(block_num)
        .map_err(|e| Error::Provider(format!("Error fetching block header: {e:?}")))?
        .ok_or_else(|| Error::Provider(String::from("Block header not found")))?;

    Ok(block_header)
}

#[derive(new, Debug, Clone)]
pub struct PreflightResult {
    pub host_output: Bytes,
    pub input: Input,
    pub gas_used: u64,
}

impl Host {
    pub fn try_new_with_components(
        providers: CachedMultiProvider,
        block_number: u64,
        chain_client: Box<dyn chain_client::Client>,
        config: Config,
    ) -> Result<Self, Error> {
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

    pub async fn preflight(self, call: Call) -> Result<PreflightResult, Error> {
        self.validate_calldata_size(&call)?;

        let SuccessfulExecutionResult {
            output: host_output,
            gas_used,
        } = panic::catch_unwind(|| {
            TravelCallExecutor::new(&self.envs).call(&call, self.start_execution_location)
        })
        .map_err(wrap_engine_panic)??;

        info!(gas_used_preflight = gas_used, "Gas used in preflight: {}", gas_used);

        let multi_evm_input =
            into_multi_input(self.envs).map_err(|err| Error::CreatingInput(err.to_string()))?;

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

        Ok(PreflightResult::new(host_output.into(), input, gas_used))
    }

    pub fn prove(
        prover: &Prover,
        call_guest_id: CallGuestId,
        PreflightResult {
            host_output, input, ..
        }: PreflightResult,
    ) -> Result<HostOutput, Error> {
        let (seal, raw_guest_output) = provably_execute(prover, &input)?;
        let proof_len = raw_guest_output.len();
        let guest_output = GuestOutput::from_outputs(&host_output, &raw_guest_output)?;

        if guest_output.evm_call_result != host_output {
            return Err(Error::HostGuestOutputMismatch(
                host_output.into(),
                guest_output.evm_call_result,
            ));
        }

        Ok(HostOutput {
            guest_output,
            seal,
            raw_abi: raw_guest_output,
            proof_len,
            call_guest_id,
        })
    }

    pub async fn main(self, call: Call) -> Result<HostOutput, Error> {
        let prover = self.prover();
        let call_guest_id = self.call_guest_id();
        let preflight_result = self.preflight(call).await?;
        Host::prove(&prover, call_guest_id, preflight_result)
    }

    fn validate_calldata_size(&self, call: &Call) -> Result<(), Error> {
        if call.data.len() > self.max_calldata_size {
            return Err(Error::CalldataTooLargeError(call.data.len()));
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

fn provably_execute(prover: &Prover, input: &Input) -> Result<(Vec<u8>, Vec<u8>), Error> {
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
        let config = Config {
            rpc_urls: test_rpc_urls(),
            start_chain_id: TEST_CHAIN_ID,
            proof_mode: ProofMode::Fake,
            ..Config::default()
        };
        let max_call_data_size = config.max_calldata_size;
        let host = Host::try_new_with_components(
            CachedMultiProvider::from_factory(EthersProviderFactory::new(test_rpc_urls())),
            0,
            Box::new(chain_client::RpcClient::new("")),
            config,
        )?;
        let call = Call {
            to: Default::default(),
            data: vec![0; max_call_data_size + 1],
        };

        assert_eq!(
            host.preflight(call).await.unwrap_err().to_string(),
            format!("Calldata too large: {} bytes", max_call_data_size + 1)
        );

        Ok(())
    }
}

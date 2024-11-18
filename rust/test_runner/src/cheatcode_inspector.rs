use alloy_primitives::U256;
use alloy_sol_types::{SolCall, SolType};
use call_engine::{
    utils::evm_call::{
        create_encoded_return_outcome, create_return_outcome, create_revert_outcome, split_calldata,
    },
    Call, HostOutput, Proof, Seal,
};
use call_guest_wrapper::GUEST_ELF;
use call_host::{get_latest_block_number, Config as HostConfig, Host};
use chain::TEST_CHAIN_ID;
use chain_client::RpcClient as RpcChainProofClient;
use foundry_config::RpcEndpoints;
use foundry_evm::revm::{
    interpreter::{CallInputs, CallOutcome},
    Database, EvmContext, Inspector,
};
use mock_chain_server::{ChainProofServerMock, EMPTY_PROOF_RESPONSE};
use provider::CachedMultiProvider;
use serde_json::json;

use crate::{
    cheatcodes::{callProverCall, getProofCall, CHEATCODE_CALL_ADDR},
    providers::{
        pending_state_provider::PendingStateProviderFactory, test_provider::TestProviderFactory,
    },
};

#[derive(Default)]
pub struct CheatcodeInspector {
    should_start_proving: bool,
    previous_proof: Option<Proof>,
    rpc_endpoints: RpcEndpoints,
}

impl CheatcodeInspector {
    pub fn new(rpc_endpoints: RpcEndpoints) -> Self {
        Self {
            rpc_endpoints,
            ..Default::default()
        }
    }
}

impl<DB: Database> Inspector<DB> for CheatcodeInspector {
    fn call(
        &mut self,
        context: &mut EvmContext<DB>,
        inputs: &mut CallInputs,
    ) -> Option<CallOutcome> {
        if self.should_start_proving {
            self.should_start_proving = false;
            return Some(self.run_host_sync(&context, inputs));
        }
        if inputs.target_address == CHEATCODE_CALL_ADDR {
            let (selector, _) = split_calldata(inputs);
            return match selector.try_into() {
                Ok(callProverCall::SELECTOR) => {
                    self.should_start_proving = true;
                    Some(create_encoded_return_outcome(&true, inputs))
                }
                Ok(getProofCall::SELECTOR) => {
                    if let Some(proof) = self.previous_proof.take() {
                        Some(create_encoded_return_outcome(&proof, inputs))
                    } else {
                        Some(create_revert_outcome("No proof available"))
                    }
                }
                _ => Some(create_revert_outcome("Unexpected vlayer cheatcode call")),
            };
        }
        None
    }
}

impl CheatcodeInspector {
    fn run_host_sync<DB: Database>(
        &mut self,
        context: &&mut EvmContext<DB>,
        inputs: &CallInputs,
    ) -> CallOutcome {
        let handle = tokio::runtime::Handle::try_current().expect("no tokio runtime");
        handle.block_on(self.run_host(context, inputs))
    }

    async fn run_host<DB: Database>(
        &mut self,
        context: &&mut EvmContext<DB>,
        inputs: &CallInputs,
    ) -> CallOutcome {
        let chain_proof_server = start_chain_proof_server().await;
        let host = create_host(context, &self.rpc_endpoints, chain_proof_server.url());
        let call_result = host
            .main(Call {
                to: inputs.target_address,
                data: inputs.input.clone().into(),
            })
            .await;

        match call_result {
            Ok(host_output) => {
                self.previous_proof = Some(Self::host_output_into_proof(&host_output));
                create_return_outcome(host_output.guest_output.evm_call_result, inputs)
            }
            Err(error) => create_revert_outcome(&format!("{:?}", error)),
        }
    }

    fn host_output_into_proof(host_output: &HostOutput) -> Proof {
        let call_assumptions = host_output.guest_output.call_assumptions.clone();

        let decoded_seal = Seal::abi_decode(&host_output.seal, true)
            .unwrap_or_else(|_| panic!("Failed to decode seal: {:x?}", host_output.seal));

        Proof {
            length: U256::from(host_output.proof_len),
            seal: decoded_seal,
            callAssumptions: call_assumptions,
            callGuestId: Default::default(),
        }
    }
}

async fn start_chain_proof_server() -> ChainProofServerMock {
    ChainProofServerMock::start(json!({}), EMPTY_PROOF_RESPONSE.clone()).await
}

fn create_host<DB: Database>(
    ctx: &EvmContext<DB>,
    rpc_endpoints: &RpcEndpoints,
    chain_proof_url: String,
) -> Host {
    let pending_state_provider_factory = PendingStateProviderFactory {
        block_number: ctx.env.block.number.try_into().unwrap(),
        state: ctx.journaled_state.state.clone(),
    };
    let provider_factory =
        TestProviderFactory::new(pending_state_provider_factory, rpc_endpoints.clone());
    let providers = CachedMultiProvider::new(provider_factory);
    let config = HostConfig {
        start_chain_id: TEST_CHAIN_ID,
        chain_proof_url,
        call_guest_elf: GUEST_ELF.clone(),
        ..Default::default()
    };
    let block_number = get_latest_block_number(&providers, config.start_chain_id)
        .expect("failed to get block number");
    let chain_proof_client = RpcChainProofClient::new(config.chain_proof_url.clone());

    Host::try_new_with_components(providers, block_number, chain_proof_client, config)
        .expect("failed to create host")
}

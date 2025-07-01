use alloy_primitives::U256;
use alloy_sol_types::{SolCall, SolType};
use call_engine::{
    Call, HostOutput, Proof, Seal,
    utils::evm_call::{
        create_encoded_return_outcome, create_return_outcome, create_revert_outcome, split_calldata,
    },
};
use call_host::{Config as HostConfig, Host};
use chain::TEST_CHAIN_ID;
use foundry_config::RpcEndpoints;
use foundry_evm::revm::{
    Database, EvmContext, Inspector,
    interpreter::{CallInputs, CallOutcome},
};
use guest_wrapper::{CALL_GUEST_ELF, CHAIN_GUEST_ELF};
use optimism::client::factory::cached;
use provider::CachedMultiProvider;

use crate::{
    cheatcodes::{CHEATCODE_CALL_ADDR, callProverCall, getProofCall, preverifyEmailCall},
    preverify_email,
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
            return Some(match selector.try_into() {
                Ok(callProverCall::SELECTOR) => {
                    self.should_start_proving = true;
                    create_encoded_return_outcome(&true, inputs)
                }
                Ok(getProofCall::SELECTOR) => {
                    if let Some(proof) = self.previous_proof.take() {
                        create_encoded_return_outcome(&proof, inputs)
                    } else {
                        create_revert_outcome("No proof available", inputs.gas_limit)
                    }
                }
                Ok(preverifyEmailCall::SELECTOR) => preverify_email::preverify_email(&inputs.input)
                    .map_or_else(
                        |err| create_revert_outcome(&err.to_string(), inputs.gas_limit),
                        |email| create_encoded_return_outcome(&email, inputs),
                    ),
                _ => create_revert_outcome("Unexpected vlayer cheatcode call", inputs.gas_limit),
            });
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
        let host = create_host(context, &self.rpc_endpoints);
        let call_result = host
            .main(Call {
                to: inputs.target_address,
                data: inputs.input.clone().into(),
                gas_limit: inputs.gas_limit,
            })
            .await;

        match call_result {
            Ok(host_output) => {
                self.previous_proof = Some(Self::host_output_into_proof(&host_output));
                create_return_outcome(host_output.guest_output.evm_call_result, inputs)
            }
            Err(error) => create_revert_outcome(&error.to_string(), inputs.gas_limit),
        }
    }

    fn host_output_into_proof(host_output: &HostOutput) -> Proof {
        let call_assumptions = host_output.guest_output.call_assumptions.clone();

        let decoded_seal = Seal::abi_decode(&host_output.seal, true)
            .unwrap_or_else(|_| panic!("Failed to decode seal: {:x?}", host_output.seal));

        let call_guest_id = host_output.call_guest_id.clone().into();

        Proof {
            length: U256::from(host_output.proof_len),
            seal: decoded_seal,
            callAssumptions: call_assumptions,
            callGuestId: call_guest_id,
        }
    }
}

fn create_host<DB: Database>(ctx: &EvmContext<DB>, rpc_endpoints: &RpcEndpoints) -> Host {
    let pending_state_provider_factory = PendingStateProviderFactory {
        block_number: ctx.env.block.number.try_into().unwrap(),
        state: ctx.journaled_state.state.clone(),
    };
    let provider_factory =
        TestProviderFactory::new(pending_state_provider_factory, rpc_endpoints.clone());
    let providers = CachedMultiProvider::from_factory(provider_factory);
    let config = HostConfig {
        call_guest_elf: CALL_GUEST_ELF.clone(),
        ..Default::default()
    };
    let block_number = providers
        .get_latest_block_number(TEST_CHAIN_ID)
        .expect("failed to get block number");
    let start_exec_location = (TEST_CHAIN_ID, block_number).into();
    let chain_proof_client =
        Box::new(chain_client::FakeClient::new(providers.clone(), CHAIN_GUEST_ELF.id));
    let op_client_factory = cached::Factory::default();

    Host::try_new(
        providers,
        start_exec_location,
        Some(chain_proof_client),
        op_client_factory,
        config,
    )
    .unwrap()
}

use alloy_sol_types::SolCall;
use call_engine::utils::evm_call::{
    create_encoded_return_outcome, create_return_outcome, create_revert_outcome, split_calldata,
};
use forge::revm::JournaledState;
use foundry_evm::revm::interpreter::{CallInputs, CallOutcome};
use foundry_evm::revm::primitives::U256;
use foundry_evm::revm::{Database, EvmContext, Inspector};

use call_engine::config::SEPOLIA_ID;
use call_engine::io::Call;
use call_host::host::config::HostConfig;
use call_host::host::Host;

use crate::cheatcodes::{callProverCall, getProofCall, Proof, CHEATCODE_CALL_ADDR};
use crate::pending_state_provider::{PendingStateProvider, PendingStateProviderFactory};

#[derive(Default)]
pub struct CheatcodeInspector {
    should_start_proving: bool,
}

impl<DB: Database> Inspector<DB> for CheatcodeInspector {
    fn call(
        &mut self,
        context: &mut EvmContext<DB>,
        inputs: &mut CallInputs,
    ) -> Option<CallOutcome> {
        if self.should_start_proving {
            self.should_start_proving = false;
            return Some(self.run_host(&context, inputs));
        }
        if inputs.target_address == CHEATCODE_CALL_ADDR {
            let (selector, _) = split_calldata(inputs);
            return match selector.try_into() {
                Ok(callProverCall::SELECTOR) => {
                    self.should_start_proving = true;
                    Some(create_encoded_return_outcome(true, inputs))
                }
                Ok(getProofCall::SELECTOR) => {
                    let dummy_proof = Proof {
                        length: U256::from(1337),
                        ..Default::default()
                    };
                    Some(create_encoded_return_outcome(dummy_proof, inputs))
                }
                _ => Some(create_revert_outcome("Unexpected vlayer cheatcode call")),
            };
        }
        None
    }
}

impl CheatcodeInspector {
    fn run_host<DB: Database>(
        &self,
        context: &&mut EvmContext<DB>,
        inputs: &mut CallInputs,
    ) -> CallOutcome {
        let host = create_host(&context.journaled_state);
        let call_result = host.run(
            Call {
                to: inputs.target_address,
                data: inputs.input.clone().into(),
            },
            None,
        );
        return match call_result {
            Ok(host_output) => {
                create_return_outcome(host_output.guest_output.evm_call_result, inputs)
            }
            Err(error) => create_revert_outcome(&format!("{:?}", error)),
        };
    }
}

fn create_host(journaled_state: &JournaledState) -> Host<PendingStateProvider> {
    Host::try_new_with_provider_factory(
        PendingStateProviderFactory {
            state: journaled_state.state.clone(),
        },
        HostConfig {
            rpc_urls: Default::default(),
            start_chain_id: SEPOLIA_ID,
        },
    )
    .expect("Failed to create host")
}

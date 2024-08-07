use alloy_sol_types::SolCall;
use forge::revm::JournaledState;
use call_engine::utils::evm_call::{
    create_raw_return_outcome, create_encoded_return_outcome, create_revert_outcome, split_calldata,
};
use foundry_evm::revm::interpreter::{CallInputs, CallOutcome};
use foundry_evm::revm::primitives::U256;
use foundry_evm::revm::{Database, EvmContext, Inspector};

use host::host::config::HostConfig;
use host::host::Host;
use vlayer_engine::config::SEPOLIA_ID;
use vlayer_engine::io::Call;

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
            let host = create_host(&context.journaled_state);
            return match host.run(Call {
                to: inputs.target_address,
                data: inputs.input.clone().into(),
            }) {
                Ok(host_output) => Some(create_raw_return_outcome(
                    host_output.guest_output.evm_call_result,
                    inputs,
                )),
                Err(error) => Some(create_revert_outcome(&format!("{:?}", error))),
            };
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

fn create_host(journaled_state: &JournaledState) -> Host<PendingStateProvider> {
    Host::try_new_with_provider_factory(
        PendingStateProviderFactory {
            state: journaled_state.clone(),
        },
        HostConfig {
            rpc_urls: Default::default(),
            chain_id: SEPOLIA_ID,
        },
    )
    .expect("Failed to create host")
}

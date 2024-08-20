use alloy_primitives::{b256, FixedBytes};
use alloy_sol_types::SolCall;
use call_engine::io::{Call, HostOutput};
use call_engine::utils::evm_call::{
    create_encoded_return_outcome, create_return_outcome, create_revert_outcome, split_calldata,
};
use call_engine::{Proof, ProofMode, Seal};
use call_host::host::config::HostConfig;
use call_host::host::Host;
use forge::revm::JournaledState;
use foundry_evm::revm::interpreter::{CallInputs, CallOutcome};
use foundry_evm::revm::primitives::U256;
use foundry_evm::revm::{Database, EvmContext, Inspector};

use call_engine::config::TESTING_CHAIN_ID;

use crate::cheatcodes::{callProverCall, getProofCall, CHEATCODE_CALL_ADDR};
use crate::pending_state_provider::{PendingStateProvider, PendingStateProviderFactory};

#[derive(Default)]
pub struct CheatcodeInspector {
    should_start_proving: bool,
    previous_proof: Option<Proof>,
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
                    if let Some(proof) = self.previous_proof.take() {
                        Some(create_encoded_return_outcome(proof, inputs))
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
    fn run_host<DB: Database>(
        &mut self,
        context: &&mut EvmContext<DB>,
        inputs: &mut CallInputs,
    ) -> CallOutcome {
        let host = create_host(&context.journaled_state);
        let call_result = host.run(Call {
            to: inputs.target_address,
            data: inputs.input.clone().into(),
        });

        match call_result {
            Ok(host_output) => {
                self.previous_proof = Some(Self::host_output_into_proof(&host_output));
                create_return_outcome(host_output.guest_output.evm_call_result.clone(), inputs)
            }
            Err(error) => create_revert_outcome(&format!("{:?}", error)),
        }
    }

    fn host_output_into_proof(host_output: &HostOutput) -> Proof {
        let mut commitment = host_output.guest_output.execution_commitment.clone();
        // For now we hardcode the settle block hash.
        commitment.settleBlockHash =
            b256!("6dee40da52db0e5ad9927dbf5fe137d0d8518f1b617b3ddc912f117bd58a49fd");

        Proof {
            seal: Seal {
                lhv: FixedBytes::<18>::new([0; 18]),
                // Set last byte to 1 to indicate fake proof mode
                rhv: FixedBytes::<18>::new([0; 18]).concat_const(FixedBytes::<1>::new([1])),
                mode: ProofMode::FAKE,
            },
            // We don't have journal data here yet, to be added later
            length: U256::ZERO,
            commitment,
        }
    }
}

fn create_host(journaled_state: &JournaledState) -> Host<PendingStateProvider> {
    Host::try_new_with_provider_factory(
        PendingStateProviderFactory {
            state: journaled_state.state.clone(),
        },
        HostConfig {
            rpc_urls: Default::default(),
            start_chain_id: TESTING_CHAIN_ID,
        },
    )
    .expect("Failed to create host")
}

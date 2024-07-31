use alloy_sol_types::SolCall;
use foundry_evm::revm::interpreter::{CallInputs, CallOutcome};
use foundry_evm::revm::primitives::U256;
use foundry_evm::revm::{Database, EvmContext, Inspector};

use vlayer_engine::utils::evm_call::{
    create_return_outcome, create_revert_outcome, split_calldata,
};

use crate::cheatcodes::{callProverCall, getProofCall, Proof, CHEATCODE_CALL_ADDR};

pub struct CheatcodeInspector {}

impl CheatcodeInspector {
    pub fn new() -> Self {
        Self {}
    }
}

impl<DB: Database> Inspector<DB> for CheatcodeInspector {
    fn call(
        &mut self,
        _context: &mut EvmContext<DB>,
        inputs: &mut CallInputs,
    ) -> Option<CallOutcome> {
        if inputs.target_address == CHEATCODE_CALL_ADDR {
            let (selector, _) = split_calldata(inputs);
            return match selector.try_into() {
                Ok(callProverCall::SELECTOR) => Some(create_return_outcome(true, inputs)),
                Ok(getProofCall::SELECTOR) => {
                    let dummy_proof = Proof {
                        length: U256::from(1337),
                        ..Default::default()
                    };
                    Some(create_return_outcome(dummy_proof, inputs))
                }
                _ => Some(create_revert_outcome("Unexpected vlayer cheatcode call")),
            };
        }
        None
    }
}

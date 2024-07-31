use crate::cheatcodes::{
    callProverCall, getProofCall, ExecutionCommitment, Proof, Seal, CHEATCODE_CALL_ADDR,
};
use alloy_sol_types::{SolCall, SolError, SolValue};
use forge::revm::interpreter::{Gas, InstructionResult};
use foundry_evm::revm::interpreter::{CallInputs, CallOutcome, InterpreterResult};
use foundry_evm::revm::primitives::{Address, FixedBytes, U256};
use foundry_evm::revm::{Database, EvmContext, Inspector};
use std::convert::Into;

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
            match inputs.input.slice(0..4).as_ref().try_into() {
                Ok(callProverCall::SELECTOR) => {
                    return Some(CallOutcome::new(
                        InterpreterResult::new(
                            InstructionResult::Return,
                            true.abi_encode().into(),
                            Gas::new(inputs.gas_limit),
                        ),
                        inputs.return_memory_offset.clone(),
                    ));
                }
                Ok(getProofCall::SELECTOR) => {
                    let proof = Proof {
                        length: U256::from(1337),
                        seal: Seal {
                            lhv: FixedBytes::new([0u8; 18]),
                            rhv: FixedBytes::new([0u8; 19]),
                        },
                        commitment: ExecutionCommitment {
                            proverContractAddress: Address::default(),
                            functionSelector: FixedBytes::new([0u8; 4]),
                            settleBlockNumber: U256::ZERO,
                            settleBlockHash: FixedBytes::new([0u8; 32]),
                        },
                    };
                    return Some(CallOutcome::new(
                        InterpreterResult::new(
                            InstructionResult::Return,
                            proof.abi_encode().into(),
                            Gas::new(inputs.gas_limit),
                        ),
                        inputs.return_memory_offset.clone(),
                    ));
                }
                _ => {
                    return Some(CallOutcome::new(
                        InterpreterResult::new(
                            InstructionResult::Revert,
                            alloy_sol_types::Revert::from("Unexpected Vlayer cheatcode call")
                                .abi_encode()
                                .into(),
                            Gas::new(inputs.gas_limit),
                        ),
                        usize::MAX..usize::MAX,
                    ));
                }
            }
        }
        None
    }
}

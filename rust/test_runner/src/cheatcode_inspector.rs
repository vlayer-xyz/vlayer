use crate::cheatcodes::{callProverCall, getProofCall, ExecutionCommitment, Proof, Seal};
use alloy_sol_types::{SolCall, SolValue};
use forge::revm::interpreter::{Gas, InstructionResult};
use foundry_evm::revm::interpreter::{CallInputs, CallOutcome, InterpreterResult};
use foundry_evm::revm::primitives::{address, Address, FixedBytes, U256};
use foundry_evm::revm::{Database, EvmContext, Inspector};
use std::convert::Into;
use tracing::warn;

pub struct CheatcodeInspector {}

const CHEATCODE_CALL_ADDR: Address = address!("e5F6E4A8da66436561059673919648CdEa4e486B");

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
                            Gas::new(0),
                        ),
                        inputs.return_memory_offset.clone(),
                    ));
                }
                Ok(getProofCall::SELECTOR) => {
                    let proof = Proof {
                        length: U256::ZERO,
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
                            Gas::new(0),
                        ),
                        inputs.return_memory_offset.clone(),
                    ));
                }
                _ => {
                    warn!("Unknown cheatcode call {}", inputs.input);
                }
            }
        }
        None
    }
}

use forge::revm::interpreter::{Gas, InstructionResult};
use foundry_evm::revm::interpreter::{CallInputs, CallOutcome, InterpreterResult};
use foundry_evm::revm::primitives::{address, Address, U256};
use foundry_evm::revm::{Database, EvmContext, Inspector};

use vlayer_engine::consts::U256_BYTES;

pub struct CheatcodeInspector {}

const CHEATCODE_CALL_ADDR: Address = address!("e5F6E4A8da66436561059673919648CdEa4e486B");

impl<DB: Database> Inspector<DB> for CheatcodeInspector {
    fn call(
        &mut self,
        _context: &mut EvmContext<DB>,
        inputs: &mut CallInputs,
    ) -> Option<CallOutcome> {
        if inputs.target_address == CHEATCODE_CALL_ADDR {
            let mock_return: [u8; U256_BYTES] = U256::ZERO.to_be_bytes();
            return Some(CallOutcome::new(
                InterpreterResult::new(InstructionResult::Return, mock_return.into(), Gas::new(0)),
                inputs.return_memory_offset.clone(),
            ));
        }
        return None;
    }
}

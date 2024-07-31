use alloy_sol_types::{SolError, SolValue};
use revm::interpreter::{CallInputs, CallOutcome, Gas, InstructionResult, InterpreterResult};

const SELECTOR_LEN: usize = 4;

pub fn split_calldata(inputs: &CallInputs) -> (&[u8], &[u8]) {
    inputs.input.split_at(SELECTOR_LEN)
}

pub fn create_return_outcome<T: SolValue>(value: T, inputs: &CallInputs) -> CallOutcome {
    CallOutcome::new(
        InterpreterResult::new(
            InstructionResult::Return,
            value.abi_encode().into(),
            Gas::new(inputs.gas_limit),
        ),
        inputs.return_memory_offset.clone(),
    )
}

pub fn create_revert_outcome(reason: &str) -> CallOutcome {
    CallOutcome::new(
        InterpreterResult::new(
            InstructionResult::Revert,
            alloy_sol_types::Revert::from(reason).abi_encode().into(),
            Gas::new(0),
        ),
        usize::MAX..usize::MAX,
    )
}

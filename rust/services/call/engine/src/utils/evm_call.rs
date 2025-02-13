use alloy_primitives::Bytes;
use alloy_sol_types::{SolError, SolValue};
use revm::{
    interpreter::{CallInputs, CallOutcome, Gas, InstructionResult, InterpreterResult},
    primitives::ExecutionResult,
};

const SELECTOR_LEN: usize = 4;

pub fn split_calldata(inputs: &CallInputs) -> (&[u8], &[u8]) {
    inputs.input.split_at(SELECTOR_LEN)
}

pub fn create_return_outcome<T: Into<Bytes>>(value: T, inputs: &CallInputs) -> CallOutcome {
    CallOutcome::new(
        InterpreterResult::new(InstructionResult::Return, value.into(), Gas::new(inputs.gas_limit)),
        inputs.return_memory_offset.clone(),
    )
}

pub fn execution_result_to_call_outcome(
    result: &ExecutionResult,
    inputs: &CallInputs,
) -> CallOutcome {
    let interpreter_result = InterpreterResult {
        result: execution_result_to_instruction_result(result),
        output: result.output().cloned().unwrap_or_default(),
        gas: gas_left(inputs.gas_limit, result.gas_used()),
    };

    CallOutcome {
        result: interpreter_result,
        memory_offset: inputs.return_memory_offset.clone(),
    }
}

fn gas_left(gas_limit: u64, gas_used: u64) -> Gas {
    let mut gas = Gas::new(gas_limit);
    if !gas.record_cost(gas_used) {
        unreachable!("gas_used cannot be higher than gas_limit");
    }
    gas
}

fn execution_result_to_instruction_result(result: &ExecutionResult) -> InstructionResult {
    match result {
        ExecutionResult::Success { reason, .. } => (*reason).into(),
        ExecutionResult::Revert { .. } => InstructionResult::Revert,
        ExecutionResult::Halt { reason, .. } => (*reason).into(),
    }
}

pub fn create_encoded_return_outcome<T: SolValue>(value: &T, inputs: &CallInputs) -> CallOutcome {
    create_return_outcome(value.abi_encode(), inputs)
}

pub fn create_revert_outcome(reason: &str, gas_limit: u64) -> CallOutcome {
    create_raw_revert_outcome(alloy_sol_types::Revert::from(reason).abi_encode().into(), gas_limit)
}

fn create_raw_revert_outcome(return_msg: Bytes, gas_limit: u64) -> CallOutcome {
    CallOutcome::new(
        InterpreterResult::new(InstructionResult::Revert, return_msg, Gas::new_spent(gas_limit)),
        usize::MAX..usize::MAX,
    )
}

use alloy_primitives::Bytes;
use alloy_rlp::Buf;
use alloy_sol_types::{decode_revert_reason, SolError, SolValue};
use revm::interpreter::{CallInputs, CallOutcome, Gas, InstructionResult, InterpreterResult};
use revm::primitives::ExecutionResult;

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
        gas: Gas::new(result.gas_used()),
    };

    CallOutcome {
        result: interpreter_result,
        memory_offset: inputs.return_memory_offset.clone(),
    }
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

pub fn format_failed_call_result(result: ExecutionResult) -> String {
    match result {
        ExecutionResult::Revert { output, .. } => {
            let reason = decode_revert_reason(output.chunk());
            reason.unwrap_or("revert: unknown reason".into())
        }
        _ => format!("{:?}", result),
    }
}

use alloy_primitives::Bytes;
use alloy_rlp::Buf;
use alloy_sol_types::{decode_revert_reason, SolError, SolValue};
use revm::interpreter::{CallInputs, CallOutcome, Gas, InstructionResult, InterpreterResult};
use revm::primitives::{ExecutionResult, HaltReason, Output, SuccessReason};

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
    let instruction_result = map_execution_result_to_instruction_result(result);
    let (output, gas_used) = extract_output_and_gas(result);
    let interpreter_result = build_interpreter_result(instruction_result, output, gas_used);

    CallOutcome {
        result: interpreter_result,
        memory_offset: inputs.return_memory_offset.clone(),
    }
}

fn map_execution_result_to_instruction_result(result: &ExecutionResult) -> InstructionResult {
    match result {
        ExecutionResult::Success { reason, .. } => match reason {
            SuccessReason::Stop => InstructionResult::Stop,
            SuccessReason::Return => InstructionResult::Return,
            SuccessReason::SelfDestruct => InstructionResult::SelfDestruct,
            SuccessReason::EofReturnContract => InstructionResult::ReturnContract,
        },
        ExecutionResult::Revert { .. } => InstructionResult::Revert,
        ExecutionResult::Halt { reason, .. } => match reason {
            HaltReason::OutOfGas(_) => InstructionResult::OutOfGas,
            HaltReason::OpcodeNotFound => InstructionResult::OpcodeNotFound,
            HaltReason::InvalidEFOpcode => InstructionResult::InvalidEFOpcode,
            HaltReason::InvalidJump => InstructionResult::InvalidJump,
            HaltReason::NotActivated => InstructionResult::NotActivated,
            HaltReason::StackUnderflow => InstructionResult::StackUnderflow,
            HaltReason::StackOverflow => InstructionResult::StackOverflow,
            HaltReason::OutOfOffset => InstructionResult::OutOfOffset,
            HaltReason::CreateCollision => InstructionResult::CreateCollision,
            HaltReason::PrecompileError => InstructionResult::PrecompileError,
            HaltReason::NonceOverflow => InstructionResult::NonceOverflow,
            HaltReason::CreateContractSizeLimit => InstructionResult::CreateContractSizeLimit,
            HaltReason::CreateContractStartingWithEF => {
                InstructionResult::CreateContractStartingWithEF
            },
            HaltReason::CreateInitCodeSizeLimit => InstructionResult::CreateInitCodeSizeLimit,
            HaltReason::OverflowPayment => InstructionResult::OverflowPayment,
            HaltReason::StateChangeDuringStaticCall => {
                InstructionResult::StateChangeDuringStaticCall
            },
            HaltReason::CallNotAllowedInsideStatic => {
                InstructionResult::CallNotAllowedInsideStatic
            },
            HaltReason::OutOfFunds => InstructionResult::OutOfFunds,
            HaltReason::CallTooDeep => InstructionResult::CallTooDeep,
            HaltReason::EofAuxDataOverflow => InstructionResult::EofAuxDataOverflow,
            HaltReason::EofAuxDataTooSmall => InstructionResult::EofAuxDataTooSmall,
            HaltReason::EOFFunctionStackOverflow => InstructionResult::EOFFunctionStackOverflow,
            HaltReason::FailedDeposit => unreachable!(
                "FailedDeposit is a part of the optimism revm feature and is not supported currently"
            ),
        },
    }
}

fn extract_output_and_gas(result: &ExecutionResult) -> (Bytes, u64) {
    match result {
        ExecutionResult::Success {
            output, gas_used, ..
        } => {
            let bytes = match output {
                Output::Create(b, _) | Output::Call(b) => b.clone(),
            };
            (bytes, *gas_used)
        }
        ExecutionResult::Revert { output, gas_used } => (output.clone(), *gas_used),
        ExecutionResult::Halt { gas_used, .. } => (Bytes::new(), *gas_used),
    }
}

fn build_interpreter_result(
    instruction_result: InstructionResult,
    output: Bytes,
    gas_used: u64,
) -> InterpreterResult {
    InterpreterResult {
        result: instruction_result,
        output,
        gas: Gas::new(gas_used),
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

use alloy_primitives::Bytes;
use alloy_rlp::Buf;
use alloy_sol_types::{decode_revert_reason, SolError, SolValue};
use revm::{
    interpreter::{CallInputs, CallOutcome, Gas, InstructionResult, InterpreterResult},
    primitives::{ExecutionResult, HaltReason, SuccessReason},
};
use thiserror::Error;

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

#[derive(Debug, Error, PartialEq)]
pub enum TransactError {
    #[error("contract execution stopped ({0:?}): No data was returned. Please check that your prover contract address is correct and the prover contract method is returning data")]
    Stop(SuccessReason),
    #[error("{0}")]
    Revert(String),
    #[error("contract execution halted: {0:?}")]
    Halt(HaltReason),
}

pub fn format_failed_call_result(result: ExecutionResult) -> TransactError {
    match result {
        ExecutionResult::Success {
            reason: SuccessReason::Return,
            ..
        } => {
            panic!("SuccessReason::Return is not a failed call result")
        }
        ExecutionResult::Success {
            reason:
                reason @ (SuccessReason::Stop
                | SuccessReason::SelfDestruct
                | SuccessReason::EofReturnContract),
            ..
        } => TransactError::Stop(reason),
        ExecutionResult::Revert { output, .. } => {
            let reason = decode_revert_reason(output.chunk());
            TransactError::Revert(reason.unwrap_or("revert: Non UTF-8 revert reason".into()))
        }
        ExecutionResult::Halt { reason, .. } => TransactError::Halt(reason),
    }
}

#[cfg(test)]
mod format_failed_call_result {

    use super::*;

    mod success {
        use revm::primitives::{Output, SuccessReason};

        use super::*;

        const fn success_result(reason: SuccessReason) -> ExecutionResult {
            ExecutionResult::Success {
                reason,
                gas_used: 0,
                gas_refunded: 0,
                logs: vec![],
                output: Output::Call(Bytes::new()),
            }
        }

        #[test]
        #[should_panic(expected = "SuccessReason::Return is not a failed call result")]
        fn return_() {
            let result = success_result(SuccessReason::Return);

            format_failed_call_result(result);
        }

        #[test]
        fn stop() {
            let result = success_result(SuccessReason::Stop);

            assert_eq!(
                format_failed_call_result(result).to_string(),
                "contract execution stopped (Stop): No data was returned. Please check that your prover contract address is correct and the prover contract method is returning data"
            );
        }
    }

    mod revert {
        use alloy_sol_types::Revert;

        use super::*;

        fn revert_result(reason: impl Into<Bytes>) -> ExecutionResult {
            ExecutionResult::Revert {
                output: reason.into(),
                gas_used: 0,
            }
        }

        #[test]
        fn revert_reason() {
            let revert = Revert::from("reason").abi_encode();
            let result = revert_result(revert);

            assert_eq!(format_failed_call_result(result).to_string(), "revert: reason");
        }

        #[test]
        fn non_utf8_reason() {
            let non_utf8_revert = Bytes::from_static(&[0xFF]);
            let result = revert_result(non_utf8_revert);

            assert_eq!(
                format_failed_call_result(result).to_string(),
                "revert: Non UTF-8 revert reason"
            );
        }
    }

    mod halt {
        use revm::primitives::{HaltReason, OutOfGasError};

        use super::*;

        const fn halt_result(reason: HaltReason) -> ExecutionResult {
            ExecutionResult::Halt {
                reason,
                gas_used: 0,
            }
        }

        #[test]
        fn basic_out_of_gas() {
            let halt = HaltReason::OutOfGas(OutOfGasError::Basic);
            let result = halt_result(halt);

            assert_eq!(
                format_failed_call_result(result).to_string(),
                "contract execution halted: OutOfGas(Basic)"
            );
        }
    }
}

use alloy_primitives::Bytes;
use alloy_rlp::Buf;
use alloy_sol_types::decode_revert_reason;
use revm::primitives::{ExecutionResult, HaltReason, SuccessReason};
use thiserror::Error;

#[derive(Debug, PartialEq, Clone)]
pub struct SuccessfulExecutionResult {
    pub output: Vec<u8>,
    pub gas_used: u64,
}

#[derive(Debug, Error, PartialEq)]
pub enum TransactError {
    #[error("contract execution stopped ({0:?}): No data was returned. Please check that your prover contract address is correct and the prover contract method is returning data")]
    Stop(SuccessReason),
    #[error("{0}")]
    Revert(String),
    #[error("{0}")]
    NonUtf8Revert(Bytes),
    #[error("contract execution halted: {0:?}")]
    Halt(HaltReason),
}

impl TryFrom<ExecutionResult> for SuccessfulExecutionResult {
    type Error = TransactError;

    fn try_from(execution_result: ExecutionResult) -> Result<Self, TransactError> {
        match execution_result {
            ExecutionResult::Success {
                reason: SuccessReason::Return,
                output,
                gas_used,
                ..
            } => Ok(Self {
                output: output.into_data().into(),
                gas_used,
            }),
            ExecutionResult::Success {
                reason:
                    reason @ (SuccessReason::Stop
                    | SuccessReason::SelfDestruct
                    | SuccessReason::EofReturnContract),
                ..
            } => Err(TransactError::Stop(reason)),
            ExecutionResult::Revert { output, .. } => {
                if let Some(reason) = decode_revert_reason(output.chunk()) {
                    Err(TransactError::Revert(reason))
                } else {
                    Err(TransactError::NonUtf8Revert(output))
                }
            }
            ExecutionResult::Halt { reason, .. } => Err(TransactError::Halt(reason)),
        }
    }
}

#[cfg(test)]
mod successful_execution_result_try_from {
    use alloy_primitives::Bytes;

    use super::*;

    mod success {
        use revm::primitives::{Output, SuccessReason};

        use super::*;

        fn success_result(reason: SuccessReason, output: impl Into<Bytes>) -> ExecutionResult {
            ExecutionResult::Success {
                reason,
                gas_used: 0,
                gas_refunded: 0,
                logs: vec![],
                output: Output::Call(output.into()),
            }
        }

        #[test]
        fn return_() -> Result<(), TransactError> {
            let result = success_result(SuccessReason::Return, [1]);
            let successful_result = SuccessfulExecutionResult::try_from(result)?;

            assert_eq!(successful_result.output, &[1]);

            Ok(())
        }

        #[test]
        fn stop() {
            let result = success_result(SuccessReason::Stop, []);
            let error = SuccessfulExecutionResult::try_from(result)
                .unwrap_err()
                .to_string();

            assert_eq!(
                error,
                "contract execution stopped (Stop): No data was returned. Please check that your prover contract address is correct and the prover contract method is returning data"
            );
        }
    }

    mod revert {
        use alloy_sol_types::{Revert, SolError};

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
            let error = SuccessfulExecutionResult::try_from(result)
                .unwrap_err()
                .to_string();

            assert_eq!(error, "revert: reason");
        }

        #[test]
        fn non_utf8_reason() {
            let non_utf8_revert = Bytes::from_static(&[0xFF]);
            let result = revert_result(non_utf8_revert);
            let error = SuccessfulExecutionResult::try_from(result)
                .unwrap_err()
                .to_string();

            assert_eq!(error, "0xff");
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
            let error = SuccessfulExecutionResult::try_from(result)
                .unwrap_err()
                .to_string();

            assert_eq!(error, "contract execution halted: OutOfGas(Basic)");
        }
    }
}

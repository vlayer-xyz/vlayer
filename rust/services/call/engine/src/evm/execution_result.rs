use alloy_primitives::Bytes;
use call_common::Metadata;
use revm::primitives::{ExecutionResult, HaltReason, SuccessReason};
use thiserror::Error;

#[derive(Debug, PartialEq, Clone)]
pub struct SuccessfulExecutionResult {
    pub output: Vec<u8>,
    pub gas_used: u64,
    pub metadata: Box<[Metadata]>,
}

#[derive(Debug, Error, PartialEq)]
pub enum TransactError {
    #[error(
        "contract execution stopped ({0:?}): No data was returned. Please check that your prover contract address is correct and the prover contract method is returning data"
    )]
    Stop(SuccessReason),
    #[error("{0}")]
    Revert(Bytes),
    #[error("contract execution halted: {0:?}")]
    Halt(HaltReason),
}

impl SuccessfulExecutionResult {
    pub fn from_execution_result(
        execution_result: ExecutionResult,
        metadata: Box<[Metadata]>,
    ) -> Result<Self, TransactError> {
        match execution_result {
            ExecutionResult::Success {
                reason: SuccessReason::Return,
                output,
                gas_used,
                ..
            } => Ok(Self {
                output: output.into_data().into(),
                gas_used,
                metadata,
            }),
            ExecutionResult::Success {
                reason:
                    reason @ (SuccessReason::Stop
                    | SuccessReason::SelfDestruct
                    | SuccessReason::EofReturnContract),
                ..
            } => Err(TransactError::Stop(reason)),
            ExecutionResult::Revert { output, .. } => Err(TransactError::Revert(output)),
            ExecutionResult::Halt { reason, .. } => Err(TransactError::Halt(reason)),
        }
    }
}

#[cfg(test)]
mod successful_execution_result_try_from {
    use alloy_primitives::Bytes;

    use super::*;

    fn from_execution_result(
        execution_result: ExecutionResult,
    ) -> Result<SuccessfulExecutionResult, TransactError> {
        SuccessfulExecutionResult::from_execution_result(execution_result, Box::new([]))
    }

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
            let successful_result = from_execution_result(result)?;

            assert_eq!(successful_result.output, &[1]);

            Ok(())
        }

        #[test]
        fn stop() {
            let result = success_result(SuccessReason::Stop, []);
            let error = from_execution_result(result).unwrap_err().to_string();

            assert_eq!(
                error,
                "contract execution stopped (Stop): No data was returned. Please check that your prover contract address is correct and the prover contract method is returning data"
            );
        }
    }

    mod revert {
        use super::*;

        #[test]
        fn non_utf8_reason() {
            let result = ExecutionResult::Revert {
                output: [0xff].into(),
                gas_used: 0,
            };
            let error = from_execution_result(result).unwrap_err();

            assert_eq!(error, TransactError::Revert([0xff].into()));
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
            let error = from_execution_result(result).unwrap_err().to_string();

            assert_eq!(error, "contract execution halted: OutOfGas(Basic)");
        }
    }
}

use alloy_primitives::Bytes;
use alloy_sol_types::GenericRevertReason;
use call_engine::{
    evm::{self},
    verifier,
};
use revm::primitives::{EVMError, HaltReason, SuccessReason};
use thiserror::Error;

use crate::{db::HostDbError, into_input};

#[derive(Error, Debug)]
pub enum Error {
    #[error("Calldata too large: {0} bytes")]
    CalldataTooLargeError(usize),

    #[error("Execution error: {0}")]
    Execution(#[from] ExecutionError),

    #[error("Creating input: {0}")]
    CreatingInput(#[from] into_input::Error),

    #[error("Travel Call verifier error: {0}")]
    Verifier(#[from] verifier::travel_call::Error),
}

#[derive(Error, Debug)]
pub enum ExecutionError {
    #[error("EVM error: {0}")]
    EvmError(#[from] EVMError<HostDbError>),

    #[error("EVM transact error: {0}")]
    TransactError(#[from] TransactError),

    #[error("Failed to get EvmEnv: {0}")]
    EvmEnv(#[from] evm::env::factory::Error),

    #[error("Panic: {0}")]
    Panic(String),
}

pub type GuestExecutionError = call_engine::travel_call::Error<HostDbError>;

impl From<GuestExecutionError> for ExecutionError {
    fn from(err: GuestExecutionError) -> Self {
        match err {
            GuestExecutionError::TransactError(err) => ExecutionError::TransactError(err.into()),
            GuestExecutionError::EvmEnv(err) => ExecutionError::EvmEnv(err),
            GuestExecutionError::EvmError(err) => ExecutionError::EvmError(err),
            GuestExecutionError::Panic(err) => ExecutionError::Panic(err),
        }
    }
}

impl From<GuestExecutionError> for Error {
    fn from(err: GuestExecutionError) -> Self {
        ExecutionError::from(err).into()
    }
}

#[derive(Debug, Error, PartialEq)]
pub enum TransactError {
    #[error("contract execution stopped ({0:?}): No data was returned. Please check that your prover contract address is correct and the prover contract method is returning data")]
    Stop(SuccessReason),
    #[error("{0}")]
    Revert(RevertError),
    #[error("contract execution halted: {0:?}")]
    Halt(HaltReason),
}

pub type GuestTransactError = call_engine::evm::execution_result::TransactError;

impl From<GuestTransactError> for TransactError {
    fn from(err: GuestTransactError) -> Self {
        match err {
            GuestTransactError::Revert(output) => TransactError::Revert(output.into()),
            GuestTransactError::Halt(reason) => TransactError::Halt(reason),
            GuestTransactError::Stop(reason) => TransactError::Stop(reason),
        }
    }
}

#[derive(Debug, Error, PartialEq)]
pub enum RevertError {
    #[error("revert: <empty>")]
    EmptyRevert,
    #[error("revert: {0:?}")]
    Revert(GenericRevertReason),
    #[error("raw bytes revert: {0}")]
    RawBytes(Bytes),
}

impl From<Bytes> for RevertError {
    fn from(bytes: Bytes) -> Self {
        if bytes.is_empty() {
            return RevertError::EmptyRevert;
        }
        match GenericRevertReason::decode(&bytes) {
            Some(reason) => RevertError::Revert(reason),
            None => RevertError::RawBytes(bytes),
        }
    }
}

#[cfg(test)]
mod revert {
    use alloy_primitives::hex::FromHex;
    use alloy_sol_types::{ContractError, PanicKind, RevertReason};

    use super::*;

    #[test]
    fn empty() {
        let raw = Bytes::default();
        let revert = RevertError::from(raw);
        assert_eq!(revert, RevertError::EmptyRevert);
    }

    #[test]
    fn raw_non_utf_8_bytes() -> anyhow::Result<()> {
        let raw = Bytes::from_hex("ff")?;
        let revert = RevertError::from(raw.clone());
        assert_eq!(revert, RevertError::RawBytes(raw));
        Ok(())
    }

    #[test]
    fn raw_string() {
        let raw = Bytes::from("text".as_bytes());
        let revert = RevertError::from(raw);
        assert_eq!(revert, RevertError::Revert(RevertReason::RawString("text".to_string())));
    }

    #[test]
    fn revert() -> anyhow::Result<()> {
        let raw = Bytes::from_hex("08c379a00000000000000000000000000000000000000000000000000000000000000020000000000000000000000000000000000000000000000000000000000000000d496e636f72726563742055524c00000000000000000000000000000000000000")?;
        let revert = RevertError::from(raw);
        assert_eq!(
            revert,
            RevertError::Revert(RevertReason::ContractError(ContractError::Revert(
                "Incorrect URL".into()
            )))
        );
        Ok(())
    }

    #[test]
    fn panik() -> anyhow::Result<()> {
        let raw = Bytes::from_hex(
            "4e487b710000000000000000000000000000000000000000000000000000000000000012",
        )?;
        let revert = RevertError::from(raw);
        assert_eq!(
            revert,
            RevertError::Revert(RevertReason::ContractError(ContractError::Panic(
                PanicKind::DivisionByZero.into()
            )))
        );
        Ok(())
    }
}

use alloy_primitives::TxKind;
use revm::{
    primitives::{ExecutionResult, ResultAndState, SuccessReason},
    Database, Evm,
};
use thiserror::Error;

use crate::{
    evm::{
        block_header::EvmBlockHeader,
        env::{EvmEnv, ExecutionLocation},
    },
    io::Call,
};

#[derive(Default)]
pub struct Engine {}

#[derive(Error, Debug, PartialEq)]
pub enum EngineError {
    #[error("EVM transact preverified error: {0}")]
    TransactPreverifiedError(String),

    #[error("EVM transact error: {0}")]
    TransactError(String),

    #[error("Unsupported chain id: {0}")]
    UnsupportedChainId(u64),

    #[error("Chain spec error: {0}")]
    ChainSpecError(String),

    #[error("EVM not found for location")]
    EvmNotFound(ExecutionLocation),
}

impl Engine {
    pub fn call<D, H>(self, tx: &Call, env: &mut EvmEnv<D, H>) -> Result<Vec<u8>, EngineError>
    where
        D: Database,
        D::Error: std::fmt::Debug,
        H: EvmBlockHeader,
    {
        let evm = Evm::builder()
            .with_db(&mut env.db)
            .with_cfg_env_with_handler_cfg(env.cfg_env.clone())
            .modify_block_env(|blk_env| env.header.fill_block_env(blk_env))
            .build();

        Self::transact(evm, tx)
    }

    fn transact<D>(mut evm: Evm<'_, (), &mut D>, tx: &Call) -> Result<Vec<u8>, EngineError>
    where
        D: Database,
        D::Error: std::fmt::Debug,
    {
        let tx_env = evm.tx_mut();
        tx_env.caller = tx.caller;
        tx_env.transact_to = TxKind::Call(tx.to);
        tx_env.data = tx.data.clone().into();

        let ResultAndState { result, .. } = evm
            .transact_preverified()
            .map_err(|err| EngineError::TransactPreverifiedError(format!("{:?}", err)))?;

        let ExecutionResult::Success {
            reason: SuccessReason::Return,
            output,
            ..
        } = result
        else {
            return Err(EngineError::TransactError(format!("{:?}", result)));
        };
        Ok(output.into_data().into())
    }
}

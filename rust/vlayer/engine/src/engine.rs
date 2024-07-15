use alloy_primitives::TxKind;
use revm::{
    inspector_handle_register,
    primitives::{ExecutionResult, ResultAndState, SuccessReason},
    Database, Evm, Inspector,
};
use thiserror::Error;

use crate::{
    evm::{
        block_header::EvmBlockHeader,
        env::{EvmEnv, ExecutionLocation},
    },
    inspector::SetInspector,
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
    pub fn call<D>(self, tx: &Call, env: &mut EvmEnv<D>) -> Result<Vec<u8>, EngineError>
    where
        D: Database,
        D::Error: std::fmt::Debug,
    {
        let evm = Evm::builder()
            .with_db(&mut env.db)
            .with_external_context(SetInspector::default())
            .with_cfg_env_with_handler_cfg(env.cfg_env.clone())
            .append_handler_register(inspector_handle_register)
            .modify_block_env(|blk_env| env.header.fill_block_env(blk_env))
            .build();

        Self::transact(evm, tx)
    }

    fn transact<D, I>(mut evm: Evm<'_, I, &mut D>, tx: &Call) -> Result<Vec<u8>, EngineError>
    where
        D: Database,
        D::Error: std::fmt::Debug,
        I: Inspector<D>,
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

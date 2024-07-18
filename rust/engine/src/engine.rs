use revm::{
    db::WrapDatabaseRef,
    inspector_handle_register,
    primitives::{ExecutionResult, ResultAndState, SuccessReason},
    DatabaseRef, Evm,
};
use thiserror::Error;

use crate::{
    block_header::EvmBlockHeader,
    evm::env::{EvmEnv, ExecutionLocation},
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

    #[error("EVM Env not found for location")]
    EvmEnvNotFound(ExecutionLocation),
}

impl Engine {
    pub fn call<D, H>(self, tx: &Call, env: &EvmEnv<D, H>) -> Result<Vec<u8>, EngineError>
    where
        D: DatabaseRef,
        D::Error: std::fmt::Debug,
        H: EvmBlockHeader,
    {
        let evm = Evm::builder()
            .with_ref_db(&env.db)
            .with_external_context(SetInspector::default())
            .with_cfg_env_with_handler_cfg(env.cfg_env.clone())
            .with_tx_env(tx.clone().into())
            .append_handler_register(inspector_handle_register)
            .modify_block_env(|blk_env| env.header.fill_block_env(blk_env))
            .build();

        Self::transact(evm)
    }

    fn transact<D>(
        mut evm: Evm<'_, SetInspector, WrapDatabaseRef<&D>>,
    ) -> Result<Vec<u8>, EngineError>
    where
        D: DatabaseRef,
        D::Error: std::fmt::Debug,
    {
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

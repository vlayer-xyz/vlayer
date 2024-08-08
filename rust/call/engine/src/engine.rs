use alloy_primitives::ChainId;
use revm::{
    db::WrapDatabaseRef,
    inspector_handle_register,
    primitives::{ExecutionResult, ResultAndState, SuccessReason},
    DatabaseRef, Evm,
};
use thiserror::Error;
use tracing::error;

use crate::{
    evm::env::{cached::CachedEvmEnv, location::ExecutionLocation},
    inspector::TravelInspector,
    io::Call,
};
use crate::{io::Augmentors, utils::evm_call::format_failed_call_result};

pub struct Engine<'a, D>
where
    D: DatabaseRef,
{
    envs: &'a CachedEvmEnv<D>,
}

#[derive(Error, Debug, PartialEq)]
pub enum EngineError {
    #[error("EVM transact preverified error: {0}")]
    TransactPreverifiedError(String),

    #[error("EVM transact error: {0}")]
    TransactError(String),

    #[error("Unsupported chain id: {0}")]
    UnsupportedChainId(ChainId),

    #[error("Chain spec error: {0}")]
    ChainSpecError(String),

    #[error("Failed to get EvmEnv: {0}")]
    EvmEnv(String),
}

impl<'a, D> Engine<'a, D>
where
    D: DatabaseRef,
    D::Error: std::fmt::Debug,
{
    pub fn new(envs: &'a CachedEvmEnv<D>) -> Self {
        Self { envs }
    }

    pub fn call(
        &'a self,
        tx: &Call,
        start_location: ExecutionLocation,
        _augmentors: Option<Augmentors>,
    ) -> Result<Vec<u8>, EngineError> {
        self.traveling_call(tx, start_location)
    }

    pub fn traveling_call(
        &'a self,
        tx: &Call,
        location: ExecutionLocation,
    ) -> Result<Vec<u8>, EngineError> {
        let env = self
            .envs
            .get(location)
            .map_err(|err| EngineError::EvmEnv(err.to_string()))?;
        let callback = |call: &_, location| self.traveling_call(call, location);
        let inspector = TravelInspector::new(env.cfg_env.chain_id, callback);
        let evm = Evm::builder()
            .with_ref_db(&env.db)
            .with_external_context(inspector)
            .with_cfg_env_with_handler_cfg(env.cfg_env.clone())
            .with_tx_env(tx.clone().into())
            .append_handler_register(inspector_handle_register)
            .modify_block_env(|blk_env| env.header.fill_block_env(blk_env))
            .build();

        Self::transact(evm)
    }

    fn transact(
        mut evm: Evm<'_, TravelInspector<'a>, WrapDatabaseRef<&D>>,
    ) -> Result<Vec<u8>, EngineError> {
        let ResultAndState { result, .. } = evm
            .transact_preverified()
            .map_err(|err| EngineError::TransactPreverifiedError(format!("{:?}", err)))?;

        let ExecutionResult::Success {
            reason: SuccessReason::Return,
            output,
            ..
        } = result
        else {
            return Err(EngineError::TransactError(format_failed_call_result(
                result,
            )));
        };
        Ok(output.into_data().into())
    }
}

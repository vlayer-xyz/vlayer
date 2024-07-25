use revm::interpreter::CallOutcome;
use revm::{
    db::WrapDatabaseRef,
    inspector_handle_register,
    interpreter::CallInputs,
    primitives::{ExecutionResult, ResultAndState, SuccessReason},
    DatabaseRef, Evm,
};
use thiserror::Error;
use tracing::{error, info};

use crate::{
    evm::env::{cached::CachedEvmEnv, location::ExecutionLocation},
    inspector::TravelInspector,
    io::Call,
};

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
    UnsupportedChainId(u64),

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

    pub fn call(&self, tx: &Call, location: ExecutionLocation) -> Result<Vec<u8>, EngineError> {
        let env = self
            .envs
            .get(location)
            .map_err(|err| EngineError::EvmEnv(err.to_string()))?;
        let callback = |location, inputs| self.inspector_callback(location, inputs);
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

    fn inspector_callback(
        &self,
        location: ExecutionLocation,
        _: CallInputs,
    ) -> Option<CallOutcome> {
        info!(
            "Intercepting the call. Block number: {:?}, chain id: {:?}",
            location.block_number, location.chain_id
        );
        None
    }

    fn transact<F>(
        mut evm: Evm<'_, TravelInspector<F>, WrapDatabaseRef<&D>>,
    ) -> Result<Vec<u8>, EngineError>
    where
        F: Fn(ExecutionLocation, CallInputs) -> Option<CallOutcome>,
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

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
    block_header::EvmBlockHeader,
    evm::env::{cached::CachedEvmEnv, location::ExecutionLocation},
    inspector::{MockCallOutcome, TravelInspector},
    io::Call,
};

pub struct Engine<'a, D, H>
where
    D: DatabaseRef,
    H: EvmBlockHeader,
{
    envs: &'a CachedEvmEnv<D, H>,
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

impl<'a, D, H> Engine<'a, D, H>
where
    D: DatabaseRef,
    H: EvmBlockHeader,
    D::Error: std::fmt::Debug,
{
    pub fn new(envs: &'a CachedEvmEnv<D, H>) -> Self {
        Self { envs }
    }

    pub fn call(self, tx: &Call, location: ExecutionLocation) -> Result<Vec<u8>, EngineError> {
        let env = self
            .envs
            .get(location)
            .map_err(|err| EngineError::EvmEnv(err.to_string()))?;
        let evm = Evm::builder()
            .with_ref_db(&env.db)
            .with_external_context(TravelInspector::new(
                env.cfg_env.chain_id,
                Self::inspector_callback,
            ))
            .with_cfg_env_with_handler_cfg(env.cfg_env.clone())
            .with_tx_env(tx.clone().into())
            .append_handler_register(inspector_handle_register)
            .modify_block_env(|blk_env| env.header.fill_block_env(blk_env))
            .build();

        Self::transact(evm)
    }

    fn inspector_callback(
        location: ExecutionLocation,
        _: &mut CallInputs,
    ) -> Option<MockCallOutcome> {
        info!(
            "Intercepting the call. Block number: {:?}, chain id: {:?}",
            location.block_number, location.chain_id
        );
        None
    }

    fn transact(
        mut evm: Evm<'_, TravelInspector, WrapDatabaseRef<&D>>,
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
            return Err(EngineError::TransactError(format!("{:?}", result)));
        };
        Ok(output.into_data().into())
    }
}

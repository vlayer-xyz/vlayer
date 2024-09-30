use std::{rc::Rc, sync::Arc};

use revm::{
    db::WrapDatabaseRef,
    inspector_handle_register,
    primitives::{EVMError, ExecutionResult, ResultAndState, SuccessReason},
    DatabaseRef, Evm, Handler,
};
use thiserror::Error;
use tracing::{debug, error};

use crate::utils::evm_call::format_failed_call_result;
use crate::{evm::env::EvmEnv, precompiles::VLAYER_PRECOMPILES};
use crate::{
    evm::env::{cached::CachedEvmEnv, location::ExecutionLocation},
    inspector::TravelInspector,
    io::Call,
};

pub struct Engine<'envs, D>
where
    D: DatabaseRef,
{
    envs: &'envs CachedEvmEnv<D>,
}

#[derive(Error, Debug, PartialEq)]
pub enum EngineError {
    #[error("EVM transact preverified error: {0}")]
    TransactPreverifiedError(String),

    #[error("EVM transact error: {0}")]
    TransactError(String),

    #[error("Chain spec error: {0}")]
    ChainSpecError(String),

    #[error("Failed to get EvmEnv: {0}")]
    EvmEnv(String),
}

impl<'envs, D> Engine<'envs, D>
where
    D: DatabaseRef,
    D::Error: std::fmt::Debug,
{
    pub fn new(envs: &'envs CachedEvmEnv<D>) -> Self {
        Self { envs }
    }

    fn get_env(&self, location: ExecutionLocation) -> Result<Rc<EvmEnv<D>>, EngineError> {
        self.envs
            .get(location)
            .map_err(|err| EngineError::EvmEnv(err.to_string()))
    }

    fn build_evm<'a>(
        env: &'envs EvmEnv<D>,
        tx: &Call,
        inspector: TravelInspector<'a>,
    ) -> Result<Evm<'a, TravelInspector<'a>, WrapDatabaseRef<&'envs D>>, EngineError> {
        let precompiles_handle_register = |handler: &mut Handler<_, _, _>| {
            let precompiles = handler.pre_execution.load_precompiles();
            handler.pre_execution.load_precompiles = Arc::new(move || {
                let mut precompiles = precompiles.clone();
                precompiles.extend(VLAYER_PRECOMPILES);
                precompiles
            });
        };

        let evm = Evm::builder()
            .with_ref_db(&env.db)
            .with_external_context(inspector)
            .with_cfg_env_with_handler_cfg(env.cfg_env.clone())
            .with_tx_env(tx.clone().into())
            .append_handler_register(precompiles_handle_register)
            .append_handler_register(inspector_handle_register)
            .modify_block_env(|blk_env| env.header.fill_block_env(blk_env))
            .build();

        Ok(evm)
    }

    pub fn call(
        &'envs self,
        tx: &Call,
        location: ExecutionLocation,
    ) -> Result<Vec<u8>, EngineError> {
        let env = self.get_env(location)?;
        let transaction_callback = |call: &_, location| self.internal_call(call, location);
        let inspector = TravelInspector::new(env.cfg_env.chain_id, transaction_callback);

        let evm = Engine::build_evm(&env, tx, inspector)?;

        Self::transact(evm)
    }

    pub fn internal_call(
        &'envs self,
        tx: &Call,
        location: ExecutionLocation,
    ) -> Result<ExecutionResult, EngineError> {
        let env = self.get_env(location)?;
        let transaction_callback = |call: &_, location| self.internal_call(call, location);
        let inspector = TravelInspector::new(env.cfg_env.chain_id, transaction_callback);

        let evm = Engine::build_evm(&env, tx, inspector)?;

        Self::internal_transact(evm)
    }

    fn transact<'env>(
        mut evm: Evm<'env, TravelInspector<'env>, WrapDatabaseRef<&'env D>>,
    ) -> Result<Vec<u8>, EngineError> {
        let ResultAndState { result, .. } = evm.transact_preverified()?;
        debug!["transact execution result: {:?}", result];

        let ExecutionResult::Success {
            reason: SuccessReason::Return,
            output,
            ..
        } = result
        else {
            return Err(EngineError::TransactError(format_failed_call_result(result)));
        };
        Ok(output.into_data().into())
    }

    fn internal_transact<'env>(
        mut evm: Evm<'env, TravelInspector<'env>, WrapDatabaseRef<&'env D>>,
    ) -> Result<ExecutionResult, EngineError> {
        let ResultAndState { result, .. } = evm.transact_preverified()?;
        debug!["internal_transact execution result: {:?}", result];
        Ok(result)
    }
}

impl<D: std::fmt::Debug> From<EVMError<D>> for EngineError {
    fn from(err: EVMError<D>) -> Self {
        match err {
            EVMError::Precompile(err) => EngineError::TransactError(format_failed_call_result({
                ExecutionResult::Revert {
                    gas_used: 0,
                    output: err.into_bytes().into(),
                }
            })),
            _ => EngineError::TransactPreverifiedError(format!("{:?}", err)),
        }
    }
}

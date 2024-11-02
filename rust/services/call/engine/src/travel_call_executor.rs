use std::sync::Arc;

use derive_new::new;
use revm::{
    db::WrapDatabaseRef,
    inspector_handle_register,
    primitives::{EVMError, ExecutionResult, ResultAndState, SuccessReason},
    DatabaseRef, Evm, Handler,
};
use tracing::{debug, error};

use crate::{
    evm::env::{cached::CachedEvmEnv, location::ExecutionLocation, EvmEnv},
    inspector::TravelInspector,
    io::Call,
    precompiles::VLAYER_PRECOMPILES,
    utils::evm_call::format_failed_call_result,
};

#[derive(new)]
pub struct TravelCallExecutor<'envs, D>
where
    D: DatabaseRef,
{
    envs: &'envs CachedEvmEnv<D>,
}

#[derive(thiserror::Error, Debug, PartialEq)]
pub enum Error {
    #[error("EVM transact preverified error: {0}")]
    TransactPreverifiedError(String),

    #[error("EVM transact error: {0}")]
    TransactError(String),

    #[error("Chain spec error: {0}")]
    ChainSpecError(String),

    #[error("Failed to get EvmEnv: {0}")]
    EvmEnv(String),

    #[error("Panic: {0}")]
    Panic(String),
}

type Result<T> = std::result::Result<T, Error>;

impl<'envs, D> TravelCallExecutor<'envs, D>
where
    D: DatabaseRef,
    D::Error: std::fmt::Debug,
{
    fn get_env(&self, location: ExecutionLocation) -> Result<Arc<EvmEnv<D>>> {
        self.envs
            .get(location)
            .map_err(|err| Error::EvmEnv(err.to_string()))
    }

    pub fn call(self, tx: &Call, location: ExecutionLocation) -> Result<SuccessfulExecutionResult> {
        self.internal_call(tx, location)?.try_into()
    }

    pub fn internal_call(
        &'envs self,
        tx: &Call,
        location: ExecutionLocation,
    ) -> Result<ExecutionResult> {
        let env = self.get_env(location)?;
        let transaction_callback = |call: &_, location| self.internal_call(call, location);
        let inspector = TravelInspector::new(env.cfg_env.chain_id, transaction_callback);
        let mut evm = build_evm(&env, tx, inspector)?;
        let ResultAndState { result, .. } = evm.transact_preverified()?;
        debug!("EVM call result: {:?}", result);

        Ok(result)
    }
}

fn build_evm<'inspector, 'envs, D>(
    env: &'envs EvmEnv<D>,
    tx: &Call,
    inspector: TravelInspector<'inspector>,
) -> Result<Evm<'inspector, TravelInspector<'inspector>, WrapDatabaseRef<&'envs D>>>
where
    D: DatabaseRef,
    D::Error: std::fmt::Debug,
{
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

impl<D: std::fmt::Debug> From<EVMError<D>> for Error {
    fn from(err: EVMError<D>) -> Self {
        match err {
            EVMError::Precompile(err) => Error::TransactError(format_failed_call_result({
                ExecutionResult::Revert {
                    gas_used: 0,
                    output: err.into_bytes().into(),
                }
            })),
            _ => Error::TransactPreverifiedError(format!("{:?}", err)),
        }
    }
}

pub struct SuccessfulExecutionResult {
    pub output: Vec<u8>,
    pub gas_used: u64,
}

impl TryFrom<ExecutionResult> for SuccessfulExecutionResult {
    type Error = Error;

    fn try_from(execution_result: ExecutionResult) -> Result<Self> {
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
            _ => Err(Error::TransactError(format_failed_call_result(execution_result))),
        }
    }
}

use std::{any::Any, panic, sync::Arc};

use call_precompiles::PRECOMPILES;
use derive_new::new;
use revm::{
    db::WrapDatabaseRef,
    inspector_handle_register,
    primitives::{EVMError, ExecutionResult, ResultAndState},
    DatabaseRef, Evm, Handler,
};
use tracing::{debug, error};

use crate::{
    evm::{
        env::{cached::CachedEvmEnv, location::ExecutionLocation, EvmEnv},
        execution_result::{SuccessfulExecutionResult, TransactError},
    },
    inspector::TravelInspector,
    io::Call,
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
    TransactError(#[from] TransactError),

    #[error("Failed to get EvmEnv: {0}")]
    EvmEnv(#[from] crate::evm::env::factory::Error),

    #[error("Panic: {0}")]
    Panic(String),
}

type Result<T> = std::result::Result<T, Error>;

impl<'envs, D> TravelCallExecutor<'envs, D>
where
    D: DatabaseRef,
    D::Error: std::fmt::Debug,
{
    pub fn call(self, tx: &Call, location: ExecutionLocation) -> Result<SuccessfulExecutionResult> {
        let result =
            panic::catch_unwind(|| self.internal_call(tx, location)).map_err(wrap_panic)??;
        Ok(result.try_into()?)
    }

    pub fn internal_call(
        &'envs self,
        tx: &Call,
        location: ExecutionLocation,
    ) -> Result<ExecutionResult> {
        let env = self.envs.get(location)?;
        let transaction_callback = |call: &_, location| self.internal_call(call, location);
        let inspector = TravelInspector::new(env.cfg_env.chain_id, transaction_callback);
        let mut evm = build_evm(&env, tx, inspector);
        // Can panic because EVM is unable to propagate errors on intercepted calls
        let ResultAndState { result, .. } = evm.transact_preverified()?;
        debug!("EVM call result: {:?}", result);

        Ok(result)
    }
}

fn wrap_panic(err: Box<dyn Any + Send>) -> Error {
    let panic_msg = err
        .downcast::<String>()
        .map(|x| *x)
        .unwrap_or("Panic occurred".to_string());
    Error::Panic(panic_msg)
}

fn build_evm<'inspector, 'envs, D>(
    env: &'envs EvmEnv<D>,
    tx: &Call,
    inspector: TravelInspector<'inspector>,
) -> Evm<'inspector, TravelInspector<'inspector>, WrapDatabaseRef<&'envs D>>
where
    D: DatabaseRef,
    D::Error: std::fmt::Debug,
{
    let precompiles_handle_register = |handler: &mut Handler<_, _, _>| {
        let precompiles = handler.pre_execution.load_precompiles();
        handler.pre_execution.load_precompiles = Arc::new(move || {
            let mut precompiles = precompiles.clone();
            precompiles.extend(PRECOMPILES);
            precompiles
        });
    };

    let mut evm = Evm::builder()
        .with_ref_db(&env.db)
        .with_external_context(inspector)
        .with_cfg_env_with_handler_cfg(env.cfg_env.clone())
        .with_tx_env(tx.clone().into())
        .append_handler_register(precompiles_handle_register)
        .append_handler_register(inspector_handle_register)
        .modify_block_env(|blk_env| env.header.fill_block_env(blk_env))
        .build();

    preload_l1_block_info(&mut evm);

    evm
}

// EVM does it on itself in transaction validation, but we use transact_preverified so we need to do it manually.
fn preload_l1_block_info<D>(evm: &mut Evm<'_, TravelInspector<'_>, WrapDatabaseRef<&D>>)
where
    D: DatabaseRef,
    D::Error: std::fmt::Debug,
{
    let spec_id = evm.spec_id();
    let l1_block_info = revm::optimism::L1BlockInfo::try_fetch(evm.db_mut(), spec_id).expect(
        "Failed to fetch L1 block info. This should not happen as we preload all necesary data in seed_cache_db_with_trusted_data",
    );
    evm.context.evm.l1_block_info = Some(l1_block_info);
}

impl<D: std::fmt::Debug> From<EVMError<D>> for Error {
    fn from(err: EVMError<D>) -> Self {
        match err {
            EVMError::Precompile(err) => TransactError::Revert(err).into(),
            _ => Error::TransactPreverifiedError(format!("{err:?}")),
        }
    }
}

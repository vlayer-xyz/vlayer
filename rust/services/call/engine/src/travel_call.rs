use std::panic;

use call_common::Database;
use derive_new::new;
use evm::build_evm;
use revm::primitives::{ExecutionResult, ResultAndState};
use tracing::{debug, info};

use crate::{
    evm::{
        env::{cached::CachedEvmEnv, location::ExecutionLocation},
        execution_result::SuccessfulExecutionResult,
    },
    io::Call,
    travel_call::error::{wrap_panic, Result},
};

mod args;
mod error;
mod evm;
mod inspector;

pub use args::Args;
pub use error::Error;
pub use inspector::Inspector;

#[derive(new)]
pub struct Executor<'envs, D: Database> {
    envs: &'envs CachedEvmEnv<D>,
}

impl<'envs, D: Database> Executor<'envs, D> {
    pub fn call(self, tx: &Call, location: ExecutionLocation) -> Result<SuccessfulExecutionResult> {
        info!("Executing top-level EVM call");
        let result =
            panic::catch_unwind(|| self.internal_call(tx, location)).map_err(wrap_panic)??;
        Ok(result.try_into()?)
    }

    pub fn internal_call(
        &'envs self,
        tx: &Call,
        location: ExecutionLocation,
    ) -> Result<ExecutionResult> {
        info!("Executing EVM call");
        let env = self.envs.get(location)?;
        let transaction_callback = |call: &_, location| self.internal_call(call, location);
        let inspector = Inspector::new(env.cfg_env.chain_id, transaction_callback);
        let mut evm = build_evm(&env, tx, inspector);
        // Can panic because EVM is unable to propagate errors on intercepted calls
        let ResultAndState { result, .. } = evm.transact_preverified()?;
        debug!("EVM call result: {:?}", result);

        Ok(result)
    }
}

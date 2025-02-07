use std::panic;

use call_common::{ExecutionLocation, RevmDB};
use derive_new::new;
use evm::build_evm;
use revm::primitives::ResultAndState;
use tracing::{debug, info};

use crate::{
    evm::{env::cached::CachedEvmEnv, execution_result::SuccessfulExecutionResult},
    io::Call,
    travel_call::{
        error::{wrap_panic, Result},
        inspector::TravelCallResult,
    },
};

mod args;
mod error;
mod evm;
mod inspector;

pub use args::Args;
pub use error::Error;
pub use inspector::Inspector;

#[derive(new)]
pub struct Executor<'envs, D: RevmDB> {
    envs: &'envs CachedEvmEnv<D>,
}

impl<'envs, D: RevmDB> Executor<'envs, D> {
    pub fn call(self, tx: &Call, location: ExecutionLocation) -> Result<SuccessfulExecutionResult> {
        info!("Executing top-level EVM call");
        let (execution_result, metadata) =
            panic::catch_unwind(|| self.internal_call(tx, location)).map_err(wrap_panic)??;
        SuccessfulExecutionResult::from_execution_result(execution_result, metadata)
            .map_err(Error::from)
    }

    fn internal_call(&'envs self, tx: &Call, location: ExecutionLocation) -> TravelCallResult {
        info!("Executing EVM call");
        let env = self.envs.get(location)?;
        let transaction_callback = |call: &_, location| self.internal_call(call, location);
        let inspector = Inspector::new(env.cfg_env.chain_id, transaction_callback);
        let mut evm = build_evm(&env, tx, inspector);
        // Can panic because EVM is unable to propagate errors on intercepted calls
        let ResultAndState { result, .. } = evm.transact_preverified()?;
        debug!("EVM call result: {result:?}");

        Ok((result, evm.context.external.into_metadata()))
    }
}

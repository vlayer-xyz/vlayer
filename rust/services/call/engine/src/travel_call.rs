use std::panic;

use call_common::{ExecutionLocation, RevmDB, WrappedRevmDBError};
use derive_new::new;
use evm::build_evm;
use inspector::TxResultWithMetadata;
use revm::primitives::ResultAndState;
use tracing::{debug, info};

use crate::{
    evm::{env::cached::CachedEvmEnv, execution_result::SuccessfulExecutionResult},
    io::Call,
    travel_call::error::wrap_panic,
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
    start: ExecutionLocation,
    is_vlayer_test: bool,
}

impl<'envs, D: RevmDB> Executor<'envs, D> {
    pub fn call(
        self,
        tx: &Call,
    ) -> Result<SuccessfulExecutionResult, Error<WrappedRevmDBError<D>>> {
        info!("Executing top-level EVM call");
        let (execution_result, metadata) =
            panic::catch_unwind(|| self.internal_call(tx, self.start)).map_err(wrap_panic)??;
        SuccessfulExecutionResult::from_execution_result(execution_result, metadata)
            .map_err(Error::from)
    }

    fn internal_call(
        &'envs self,
        tx: &Call,
        location: ExecutionLocation,
    ) -> Result<TxResultWithMetadata, Error<WrappedRevmDBError<D>>> {
        info!("Executing EVM call");
        self.ensure_no_forward_jump(location)?;
        let env = self.envs.get(location)?;
        let transaction_callback = |call: &_, location| self.internal_call(call, location);
        let inspector = Inspector::new(
            env.cfg_env.chain_id,
            transaction_callback,
            self.is_vlayer_test,
            self.is_on_historic_block(location),
        );
        let mut evm = build_evm(&env, tx, inspector, self.is_vlayer_test);
        // Can panic because EVM is unable to propagate errors on intercepted calls
        let ResultAndState { result, .. } = evm.transact_preverified()?;
        debug!("EVM call result: {result:?}");

        Ok((result, evm.context.external.into_metadata()))
    }

    // Forward jumps are invalid as we only verify the start location on-chain
    const fn ensure_no_forward_jump(
        &self,
        location: ExecutionLocation,
    ) -> Result<(), Error<WrappedRevmDBError<D>>> {
        if location.chain_id == self.start.chain_id
            && location.block_number > self.start.block_number
        {
            return Err(Error::TimeTravelIntoFuture {
                start: self.start.block_number,
                target: location.block_number,
            });
        }
        Ok(())
    }

    const fn is_on_historic_block(&self, location: ExecutionLocation) -> bool {
        location.block_number < self.start.block_number || location.chain_id != self.start.chain_id
    }
}

#[cfg(test)]
mod tests {
    use revm::InMemoryDB;

    use super::*;

    #[test]
    fn backward_jump_is_allowed() {
        let start = ExecutionLocation::new(1, 1);
        let target = ExecutionLocation::new(1, 0);
        let envs = CachedEvmEnv::<InMemoryDB>::from_envs(Default::default());
        let executor = Executor::new(&envs, start, true);
        executor.ensure_no_forward_jump(target).unwrap();
    }

    #[test]
    fn forward_jump_is_dissalowed() {
        let start = ExecutionLocation::new(1, 0);
        let target = ExecutionLocation::new(1, 1);
        let envs = CachedEvmEnv::<InMemoryDB>::from_envs(Default::default());
        let executor = Executor::new(&envs, start, true);
        assert_eq!(
            executor.ensure_no_forward_jump(target).unwrap_err(),
            Error::TimeTravelIntoFuture {
                start: 0,
                target: 1
            }
        );
    }
}

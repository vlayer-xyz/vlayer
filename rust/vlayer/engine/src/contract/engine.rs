use revm::{
    primitives::{
        CfgEnvWithHandlerCfg, ExecutionResult, ResultAndState, SuccessReason, TransactTo,
    },
    Database, Evm,
};
use std::fmt::Debug;
use thiserror::Error;

use crate::{guest::Call, EvmBlockHeader, EvmEnv};

pub struct Engine {}

#[derive(Error, Debug, PartialEq)]
pub enum EngineError {
    #[error("EVM transact preverified error: {0}")]
    TransactPreverifiedError(String),
    #[error("EVM transact error: {0}")]
    TransactError(String),
}

impl Engine {
    pub fn call<DB, H>(tx: &Call, env: &mut EvmEnv<DB, H>) -> Result<Vec<u8>, EngineError>
    where
        DB: Database,
        DB::Error: Debug,
        H: EvmBlockHeader,
    {
        let cfg: CfgEnvWithHandlerCfg = env.cfg_env.clone();

        let evm = Evm::builder()
            .with_db(&mut env.db)
            .with_cfg_env_with_handler_cfg(cfg)
            .modify_block_env(|blk_env| env.header.fill_block_env(blk_env))
            .build();

        Self::transact(evm, tx)
    }

    fn transact<DB>(mut evm: Evm<'_, (), DB>, tx: &Call) -> Result<Vec<u8>, EngineError>
    where
        DB: Database,
        DB::Error: Debug,
    {
        let tx_env = evm.tx_mut();
        tx_env.caller = tx.caller;
        tx_env.transact_to = TransactTo::call(tx.to);
        tx_env.data = tx.data.clone().into();

        let ResultAndState { result, .. } = evm
            .transact_preverified()
            .map_err(|err| EngineError::TransactPreverifiedError(format!("{:?}", err)))?;

        if let ExecutionResult::Success {
            reason: SuccessReason::Return,
            output,
            ..
        } = result
        {
            Ok(output.into_data().into())
        } else {
            Err(EngineError::TransactError(format!("{:?}", result)))
        }
    }
}

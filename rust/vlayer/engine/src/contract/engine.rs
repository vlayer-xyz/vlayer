use revm::{
    primitives::{
        CfgEnvWithHandlerCfg, ExecutionResult, ResultAndState, SuccessReason, TransactTo,
    },
    Database, Evm,
};
use std::fmt::Debug;
use thiserror::Error;

use crate::{guest::Call, EvmBlockHeader, EvmEnv};

pub struct Engine<'a, D, H> {
    env: &'a mut EvmEnv<D, H>,
}

#[derive(Error, Debug, PartialEq)]
pub enum EngineError {
    #[error("EVM transact preverified error: {0}")]
    TransactPreverifiedError(String),
    #[error("EVM transact error: {0}")]
    TransactError(String),
}

impl<'a, D, H> Engine<'a, D, H>
where
    D: Database,
    D::Error: Debug,
    H: EvmBlockHeader,
{
    pub fn new(env: &'a mut EvmEnv<D, H>) -> Self {
        Engine { env }
    }

    pub fn call(self, tx: &Call) -> anyhow::Result<Vec<u8>> {
        let cfg: CfgEnvWithHandlerCfg = self.env.cfg_env.clone();

        let evm = Evm::builder()
            .with_db(&mut self.env.db)
            .with_cfg_env_with_handler_cfg(cfg)
            .modify_block_env(|blk_env| self.env.header.fill_block_env(blk_env))
            .build();

        Self::transact(evm, tx)
    }

    fn transact(mut evm: Evm<'_, (), &mut D>, tx: &Call) -> Result<Vec<u8>, EngineError>
    where
        D: Database,
        D::Error: Debug,
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

use revm::{
    primitives::{ExecutionResult, ResultAndState, SuccessReason, TransactTo},
    Database, Evm,
};
use std::fmt::Debug;
use thiserror::Error;

use crate::{config::CHAIN_MAP, guest::Call, EvmBlockHeader, EvmEnv};

pub struct Engine<D, H> {
    env: EvmEnv<D, H>,
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
}

impl<D, H> Engine<D, H>
where
    D: Database,
    D::Error: std::fmt::Debug,
    H: EvmBlockHeader,
{
    pub fn try_new(db: D, header: H, chain_id: u64) -> Result<Self, EngineError> {
        let chain_spec = CHAIN_MAP
            .get(&chain_id)
            .ok_or(EngineError::UnsupportedChainId(chain_id))?;

        let env = EvmEnv::new(db, header.seal_slow())
            .with_chain_spec(chain_spec)
            .map_err(|err| EngineError::ChainSpecError(err.to_string()))?;
        Ok(Engine { env })
    }

    pub fn call(mut self, tx: &Call) -> Result<Vec<u8>, EngineError> {
        let evm = Evm::builder()
            .with_db(&mut self.env.db)
            .with_cfg_env_with_handler_cfg(self.env.cfg_env)
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

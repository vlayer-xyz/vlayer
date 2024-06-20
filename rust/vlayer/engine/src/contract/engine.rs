use anyhow::anyhow;
use revm::{
    primitives::{
        CfgEnvWithHandlerCfg, ExecutionResult, ResultAndState, SuccessReason, TransactTo,
    },
    Database, Evm,
};

use crate::{guest::Call, EvmBlockHeader, EvmEnv};

pub struct Engine<'a, D> {
    db: &'a mut D,
}

impl<'a, D: Database + 'a> Engine<'a, D> {
    pub fn call<H>(tx: &Call, env: &mut EvmEnv<D, H>) -> anyhow::Result<Vec<u8>>
    where
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

    fn transact(mut evm: Evm<'_, (), &mut D>, tx: &Call) -> anyhow::Result<Vec<u8>> {
        let tx_env = evm.tx_mut();
        tx_env.caller = tx.caller;
        tx_env.transact_to = TransactTo::call(tx.to);
        tx_env.data = tx.data.clone().into();

        let ResultAndState { result, .. } = evm
            .transact_preverified()
            .map_err(|_| anyhow!("Transact error"))?;
        let ExecutionResult::Success {
            reason: SuccessReason::Return,
            output,
            ..
        } = result
        else {
            return Err(anyhow!("Call did not return: "));
        };

        Ok(output.into_data().into())
    }
}

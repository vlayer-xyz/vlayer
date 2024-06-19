use super::{db::WrapStateDb, new_evm, transact};
use anyhow::anyhow;
use revm::{
    primitives::{
        CfgEnvWithHandlerCfg, ExecutionResult, ResultAndState, SuccessReason, TransactTo,
    },
    Database, Evm,
};

#[cfg(feature = "host")]
use crate::host::{provider::Provider, HostEvmEnv};
use crate::{guest::Call, EvmBlockHeader, EvmEnv, GuestEvmEnv};

pub struct Engine {}

impl Engine {
    #[cfg(feature = "host")]
    pub fn evm_call<P, H>(tx: &Call, env: &mut HostEvmEnv<P, H>) -> anyhow::Result<Vec<u8>>
    where
        P: Provider,
        H: EvmBlockHeader,
    {
        log::info!("Executing preflight for on contract {}", tx.to);

        let evm = new_evm(&mut env.db, env.cfg_env.clone(), &env.header);

        transact(evm, tx).map_err(|err| anyhow::anyhow!(err))
    }

    pub fn guest_evm_call<H>(tx: &Call, env: &GuestEvmEnv<H>) -> Vec<u8>
    where
        H: EvmBlockHeader,
    {
        let evm = new_evm(WrapStateDb::new(&env.db), env.cfg_env.clone(), &env.header);
        #[allow(clippy::unwrap_used)]
        transact(evm, tx).unwrap()
    }

    pub fn call<DB, H>(tx: &Call, env: &mut EvmEnv<DB, H>) -> anyhow::Result<Vec<u8>>
    where
        DB: Database,
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

    fn transact<DB>(mut evm: Evm<'_, (), DB>, tx: &Call) -> anyhow::Result<Vec<u8>>
    where
        DB: Database,
    {
        let tx_env = evm.tx_mut();
        tx_env.caller = tx.caller;
        tx_env.transact_to = TransactTo::call(tx.to);
        tx_env.data = tx.data.clone().into();

        let ResultAndState { result, .. } = evm
            .transact_preverified()
            .map_err(|_err| anyhow!("Transact error"))?;
        let ExecutionResult::Success { reason, output, .. } = result else {
            return Err(anyhow!("Call did not return: "));
        };

        if reason != SuccessReason::Return {
            return Err(anyhow!("Call did not return: "));
        }

        Ok(output.into_data().into())
    }
}

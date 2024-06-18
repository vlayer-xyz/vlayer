use super::{db::WrapStateDb, new_evm, transact, CallTxData};

#[cfg(feature = "host")]
use crate::host::{provider::Provider, HostEvmEnv};
use crate::{EvmBlockHeader, GuestEvmEnv};

#[cfg(feature = "host")]
pub fn evm_call<P, H>(tx: CallTxData, env: &mut HostEvmEnv<P, H>) -> anyhow::Result<Vec<u8>>
where
    P: Provider,
    H: EvmBlockHeader,
{
    log::info!("Executing preflight for on contract {}", tx.to);

    let evm = new_evm(&mut env.db, env.cfg_env.clone(), &env.header);

    transact(evm, tx).map_err(|err| anyhow::anyhow!(err))
}

pub fn guest_evm_call<H>(tx: CallTxData, env: &GuestEvmEnv<H>) -> Vec<u8>
where
    H: EvmBlockHeader,
{
    let evm = new_evm(WrapStateDb::new(&env.db), env.cfg_env.clone(), &env.header);
    #[allow(clippy::unwrap_used)]
    transact(evm, tx).unwrap()
}

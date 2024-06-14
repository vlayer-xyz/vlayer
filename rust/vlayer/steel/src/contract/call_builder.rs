use super::{db::WrapStateDb, new_evm, transact, CallTxData};

#[cfg(feature = "host")]
use crate::host::{provider::Provider, HostEvmEnv};
use crate::{EvmBlockHeader, GuestEvmEnv};
use alloy_primitives::{Address, U256};
use alloy_sol_types::SolCall;
use std::marker::PhantomData;

/// A builder for calling an Ethereum contract.
///
/// Once configured, call with [CallBuilder::call].
#[derive(Debug, Clone)]
#[must_use]
pub struct CallBuilder<C> {
    tx: CallTxData<C>,
}

impl<C> CallBuilder<C> {
    /// The default gas limit for function calls.
    const DEFAULT_GAS_LIMIT: u64 = 30_000_000;

    /// Creates a new builder for the given contract call.
    pub fn new(address: Address, call: &C) -> Self
    where
        C: SolCall,
    {
        let tx = CallTxData {
            caller: address, // by default the contract calls itself
            gas_limit: Self::DEFAULT_GAS_LIMIT,
            gas_price: U256::ZERO,
            to: address,
            value: U256::ZERO,
            data: call.abi_encode(),
            phantom: PhantomData,
        };
        Self { tx }
    }

    /// Sets the caller of the function call.
    pub fn from(mut self, from: Address) -> Self {
        self.tx.caller = from;
        self
    }

    /// Sets the gas limit of the function call.
    pub fn gas(mut self, gas: u64) -> Self {
        self.tx.gas_limit = gas;
        self
    }

    /// Sets the gas price of the function call.
    pub fn gas_price(mut self, gas_price: U256) -> Self {
        self.tx.gas_price = gas_price;
        self
    }

    /// Sets the value field of the function call.
    pub fn value(mut self, value: U256) -> Self {
        self.tx.value = value;
        self
    }
}

impl<C> From<CallTxData<C>> for CallBuilder<C> {
    fn from(tx: CallTxData<C>) -> Self {
        Self { tx }
    }
}

impl<C> From<CallBuilder<C>> for CallTxData<C> {
    fn from(builder: CallBuilder<C>) -> Self {
        builder.tx
    }
}

#[cfg(feature = "host")]
pub fn evm_call<'a, C, P, H>(
    call_builder: CallBuilder<C>,
    env: &'a mut HostEvmEnv<P, H>,
) -> anyhow::Result<C::Return>
where
    C: SolCall,
    P: Provider,
    H: EvmBlockHeader,
{
    log::info!(
        "Executing preflight for '{}' on contract {}",
        C::SIGNATURE,
        call_builder.tx.to
    );

    let evm = new_evm(&mut env.db, env.cfg_env.clone(), &env.header);

    transact(evm, call_builder.tx).map_err(|err| anyhow::anyhow!(err))
}

pub fn guest_evm_call<'a, C, H>(call_builder: CallBuilder<C>, env: &'a GuestEvmEnv<H>) -> C::Return
where
    C: SolCall,
    H: EvmBlockHeader,
{
    let evm = new_evm(WrapStateDb::new(&env.db), env.cfg_env.clone(), &env.header);
    transact(evm, call_builder.tx).unwrap()
}

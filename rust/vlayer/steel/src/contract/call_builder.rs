use super::{db::WrapStateDb, new_evm, CallTxData};

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
pub struct CallBuilder<C, E> {
    tx: CallTxData<C>,
    env: E,
}

impl<C, E> CallBuilder<C, E> {
    /// The default gas limit for function calls.
    const DEFAULT_GAS_LIMIT: u64 = 30_000_000;

    /// Creates a new builder for the given contract call.
    pub(crate) fn new(env: E, address: Address, call: &C) -> Self
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
        Self { tx, env }
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

#[cfg(feature = "host")]
impl<'a, C, P, H> CallBuilder<C, &'a mut HostEvmEnv<P, H>>
where
    C: SolCall,
    P: Provider,
    H: EvmBlockHeader,
{
    /// Executes the call with a [EvmEnv] constructed with [Contract::preflight].
    ///
    /// [EvmEnv]: crate::EvmEnv
    pub fn call(self) -> anyhow::Result<C::Return> {
        log::info!(
            "Executing preflight for '{}' on contract {}",
            C::SIGNATURE,
            self.tx.to
        );

        let evm = new_evm(&mut self.env.db, self.env.cfg_env.clone(), &self.env.header);
        self.tx.transact(evm).map_err(|err| anyhow::anyhow!(err))
    }
}

impl<'a, C, H> CallBuilder<C, &'a GuestEvmEnv<H>>
where
    C: SolCall,
    H: EvmBlockHeader,
{
    /// Executes the call with a [EvmEnv] constructed with [Contract::new].
    ///
    /// [EvmEnv]: crate::EvmEnv
    pub fn call(self, evm: revm::Evm<(), WrapStateDb>) -> C::Return {
        self.tx.transact(evm).unwrap()
    }
}

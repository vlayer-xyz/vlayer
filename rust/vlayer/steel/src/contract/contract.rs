use super::call_builder::CallBuilder;
#[cfg(feature = "host")]
use crate::host::{provider::Provider, HostEvmEnv};
use crate::GuestEvmEnv;
use alloy_primitives::Address;
use alloy_sol_types::SolCall;

pub struct Contract<E> {
    address: Address,
    env: E,
}

impl<'a, H> Contract<&'a GuestEvmEnv<H>> {
    /// Constructor for executing calls to an Ethereum contract in the guest.
    pub fn new(address: Address, env: &'a GuestEvmEnv<H>) -> Self {
        Self { address, env }
    }

    /// Initializes a call builder to execute a call on the contract.
    pub fn call_builder<C: SolCall>(&self, call: &C) -> CallBuilder<C, &GuestEvmEnv<H>> {
        CallBuilder::new(self.env, self.address, call)
    }
}

#[cfg(feature = "host")]
impl<'a, P, H> Contract<&'a mut HostEvmEnv<P, H>>
where
    P: Provider,
{
    /// Constructor for preflighting calls to an Ethereum contract on the host.
    ///
    /// Initializes the environment for calling functions on the Ethereum contract, fetching
    /// necessary data via the [Provider], and generating a storage proof for any accessed
    /// elements using [EvmEnv::into_input].
    ///
    /// [Provider]: crate::host::provider::Provider
    /// [EvmEnv::into_input]: crate::EvmEnv::into_input
    /// [EvmEnv]: crate::EvmEnv
    pub fn preflight(address: Address, env: &'a mut HostEvmEnv<P, H>) -> Self {
        Self { address, env }
    }

    /// Initializes a call builder to execute a call on the contract.
    pub fn call_builder<C: SolCall>(&mut self, call: &C) -> CallBuilder<C, &mut HostEvmEnv<P, H>> {
        CallBuilder::new(self.env, self.address, call)
    }
}

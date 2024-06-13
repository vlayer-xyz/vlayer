use super::call_builder::CallBuilder;
use alloy_primitives::Address;
use alloy_sol_types::SolCall;

pub struct Contract {
    address: Address,
}

impl Contract {
    /// Constructor for executing calls to an Ethereum contract in the guest.
    pub fn new(address: Address) -> Self {
        Self { address }
    }

    /// Initializes a call builder to execute a call on the contract.
    pub fn call_builder<C: SolCall>(&self, call: &C) -> CallBuilder<C> {
        CallBuilder::new(self.address, call)
    }
}

#[cfg(feature = "host")]
impl Contract {
    /// Constructor for preflighting calls to an Ethereum contract on the host.
    ///
    /// Initializes the environment for calling functions on the Ethereum contract, fetching
    /// necessary data via the [Provider], and generating a storage proof for any accessed
    /// elements using [EvmEnv::into_input].
    ///
    /// [Provider]: crate::host::provider::Provider
    /// [EvmEnv::into_input]: crate::EvmEnv::into_input
    /// [EvmEnv]: crate::EvmEnv
    pub fn preflight(address: Address) -> Self {
        Self { address }
    }
}

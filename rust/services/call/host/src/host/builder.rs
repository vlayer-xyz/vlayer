//! Host builder is a state machine with five linear stages.
//! Most stage transitions are fallible, some are asynchronous.
//!
//! `New`
//!  ║ `with_rpc_urls` - create `CachedMultiProvider` from URLs
//!  ║
//!  ╚>`WithProviders`
//!     ║ `with_chain_guest_id` – add chain guest ELF ID to context
//!     ║
//!     ╚> `WithChainGuestId`
//!         ║ `with_chain_proof_url` - create chain proof client from the given URL
//!         ║ or mock it using RPC providers abd chain guest ELF ID
//!         ║
//!         ╚>`WithChainClient`
//!            ║ `with_start_chain_id` - sets the chain ID where the execution starts
//!            ║
//!            ╚>`WithStartChainId`
//!               ║ `with_prover_contract_addr` - calculate start execution location,
//!               ║ (ensuring that the prover contract is deployed on that location)
//!               ║
//!               ╚>`WithStartExecLocation`
//!                  ║ `build`
//!                  ║
//!                  ╚══>`Host`

use std::collections::HashMap;

use alloy_primitives::ChainId;
use call_common::ExecutionLocation;
use provider::{Address, CachedMultiProvider, EthersProviderFactory};

use super::{BuilderError as Error, Config, Host};

pub struct New;

pub struct WithProviders {
    providers: CachedMultiProvider,
}

pub struct WithChainGuestId {
    providers: CachedMultiProvider,
}

pub struct WithChainClient {
    providers: CachedMultiProvider,
}

pub struct WithStartChainId {
    start_chain_id: ChainId,
    providers: CachedMultiProvider,
}

pub struct WithStartExecLocation {
    start_exec_location: ExecutionLocation,
    providers: CachedMultiProvider,
}

impl New {
    #[allow(clippy::unused_self)]
    #[must_use]
    pub fn with_rpc_urls(self, rpc_urls: HashMap<ChainId, String>) -> WithProviders {
        let provider_factory = EthersProviderFactory::new(rpc_urls.clone());
        let providers = CachedMultiProvider::from_factory(provider_factory);
        WithProviders { providers }
    }
}

impl WithProviders {
    pub fn with_nothing(self) -> WithChainGuestId {
        WithChainGuestId {
            providers: self.providers,
        }
    }
}

impl WithChainGuestId {
    pub fn with_chain_proof_url(self) -> Result<WithChainClient, Error> {
        let WithChainGuestId { providers } = self;
        Ok(WithChainClient { providers })
    }
}

impl WithChainClient {
    pub fn with_start_chain_id(self, start_chain_id: ChainId) -> Result<WithStartChainId, Error> {
        let WithChainClient { providers } = self;
        Ok(WithStartChainId {
            start_chain_id,
            providers,
        })
    }
}

impl WithStartChainId {
    /// Calculate the start execution location based on:
    ///   a) prover contract address,
    ///   b) latest block from RPC provider,
    ///   c) latest block indexed by chain service.
    ///
    /// There are 3 possible outcomes:
    ///   1) Prover contract is not deployed on latest RPC block --> return error,
    ///   2) Prover contract is deployed on latest RPC block & latest indexed block
    ///      --> use latest indexed block as starting location,
    ///   3) Prover contract is deployed on latest RPC block, but not latest indexed block
    ///      --> use latest RPC block as starting location (it will be necessary to wait
    ///      for this block to be indexed by the chain service).
    pub fn with_prover_contract_addr(self, _: Address) -> Result<WithStartExecLocation, Error> {
        let WithStartChainId {
            start_chain_id,
            providers,
        } = self;

        let latest_rpc_block = providers.get_latest_block_number(start_chain_id)?;
        let start_exec_location = (start_chain_id, latest_rpc_block).into();

        Ok(WithStartExecLocation {
            start_exec_location,
            providers,
        })
    }
}

impl WithStartExecLocation {
    pub fn build(self, config: Config) -> Result<Host, Error> {
        let WithStartExecLocation {
            start_exec_location,
            providers,
        } = self;
        Host::try_new(providers, start_exec_location, config)
    }
}

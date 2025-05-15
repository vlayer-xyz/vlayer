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
//!         ║ `with_chain_client_config` - create chain proof client from the given config
//!         ║ or mock it using RPC providers and chain guest ELF ID
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
use chain_client::ChainClientConfig;
use provider::{Address, BlockNumber, CachedMultiProvider, EthersProviderFactory};
use risc0_zkvm::sha::Digest;
use tracing::warn;

use super::{BuilderError as Error, Config, Host};

pub struct New;

pub struct WithProviders {
    providers: CachedMultiProvider,
    op_client_factory: Box<dyn optimism::client::IFactory>,
}

pub struct WithChainGuestId {
    providers: CachedMultiProvider,
    op_client_factory: Box<dyn optimism::client::IFactory>,
    chain_guest_id: Digest,
}

pub struct WithChainClient {
    chain_client: Box<dyn chain_client::Client>,
    providers: CachedMultiProvider,
    op_client_factory: Box<dyn optimism::client::IFactory>,
}

pub struct WithStartChainId {
    start_chain_id: ChainId,
    chain_client: Box<dyn chain_client::Client>,
    providers: CachedMultiProvider,
    op_client_factory: Box<dyn optimism::client::IFactory>,
}

pub struct WithStartExecLocation {
    chain_client: Option<Box<dyn chain_client::Client>>,
    start_exec_location: ExecutionLocation,
    providers: CachedMultiProvider,
    op_client_factory: Box<dyn optimism::client::IFactory>,
}

impl New {
    #[allow(clippy::unused_self)]
    #[must_use]
    pub fn with_rpc_urls(self, rpc_urls: HashMap<ChainId, String>) -> WithProviders {
        let provider_factory = EthersProviderFactory::new(rpc_urls.clone());
        let providers = CachedMultiProvider::from_factory(provider_factory);
        let op_client_factory = Box::new(optimism::client::factory::http::Factory::new(rpc_urls));
        WithProviders {
            providers,
            op_client_factory,
        }
    }
}

impl WithProviders {
    pub fn with_chain_guest_id(self, chain_guest_id: Digest) -> WithChainGuestId {
        WithChainGuestId {
            providers: self.providers,
            op_client_factory: self.op_client_factory,
            chain_guest_id,
        }
    }
}

impl WithChainGuestId {
    pub fn with_chain_client_config(
        self,
        chain_client_config: Option<ChainClientConfig>,
    ) -> Result<WithChainClient, Error> {
        let WithChainGuestId {
            providers,
            chain_guest_id,
            op_client_factory,
        } = self;
        let chain_client: Box<dyn chain_client::Client> = match chain_client_config {
            Some(config) => Box::new(chain_client::RpcClient::new(&config)),
            None => {
                warn!("Chain client config not provided. Running with mock server");
                Box::new(chain_client::FakeClient::new(providers.clone(), chain_guest_id))
            }
        };
        Ok(WithChainClient {
            chain_client,
            providers,
            op_client_factory,
        })
    }
}

impl WithChainClient {
    pub fn with_start_chain_id(self, start_chain_id: ChainId) -> Result<WithStartChainId, Error> {
        let WithChainClient {
            chain_client,
            providers,
            op_client_factory,
        } = self;
        Ok(WithStartChainId {
            start_chain_id,
            providers,
            chain_client,
            op_client_factory,
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
    pub async fn with_prover_contract_addr(
        self,
        prover_contract_addr: Address,
    ) -> Result<WithStartExecLocation, Error> {
        let WithStartChainId {
            start_chain_id,
            chain_client,
            providers,
            op_client_factory,
        } = self;

        let prover_contract_deployed =
            check_prover_contract(&providers, start_chain_id, prover_contract_addr);

        let latest_rpc_block = providers.get_latest_block_number(start_chain_id)?;
        if !prover_contract_deployed(latest_rpc_block)? {
            return Err(Error::ProverContractNotDeployed(prover_contract_addr, latest_rpc_block));
        }

        let sync_status = chain_client.get_sync_status(start_chain_id).await;
        let Ok(sync_status) = sync_status else {
            // `prover_contract_deployed`` borrows `providers`` and borrow checker only works on code blocks level and not on lines level
            // Therefore we need to drop manually before we can return providers
            drop(prover_contract_deployed);
            // If chain service is not available, we fallback to a degraded mode (no teleport or time travel)
            return Ok(WithStartExecLocation {
                chain_client: None,
                start_exec_location: (start_chain_id, latest_rpc_block).into(),
                providers,
                op_client_factory,
            });
        };

        let start_block_number =
            compute_start_block_number(latest_rpc_block, prover_contract_deployed, &sync_status)?;
        let start_exec_location = (start_chain_id, start_block_number).into();

        Ok(WithStartExecLocation {
            chain_client: Some(chain_client),
            start_exec_location,
            providers,
            op_client_factory,
        })
    }
}

fn compute_start_block_number(
    latest_rpc_block: BlockNumber,
    prover_contract_deployed: impl Fn(BlockNumber) -> Result<bool, Error>,
    sync_status: &chain_common::SyncStatus,
) -> Result<BlockNumber, Error> {
    if prover_contract_deployed(sync_status.last_block)? {
        Ok(sync_status.last_block)
    } else {
        Ok(latest_rpc_block)
    }
}

impl WithStartExecLocation {
    pub fn build(self, config: Config) -> Result<Host, Error> {
        let WithStartExecLocation {
            chain_client,
            start_exec_location,
            providers,
            op_client_factory,
        } = self;
        Host::try_new(providers, start_exec_location, chain_client, op_client_factory, config)
    }
}

fn check_prover_contract(
    provider: &CachedMultiProvider,
    chain_id: ChainId,
    address: Address,
) -> impl Fn(BlockNumber) -> Result<bool, Error> + '_ {
    move |block_num| Ok(!provider.get_code(chain_id, address, block_num)?.is_empty())
}

#[cfg(test)]
mod tests {
    use super::*;

    mod start_exec_location {
        use std::sync::Arc;

        use chain_client::PartiallySyncedClient;
        use chain_common::SyncStatus;
        use ethers_core::types::{Bytes, U64};
        use ethers_providers::MockProvider;
        use optimism::client::factory::cached;
        use provider::{BlockingProvider, EthersProvider};

        use super::*;

        const CHAIN_ID: ChainId = 1;
        const LATEST_INDEXED_BLOCK: BlockNumber = 1;
        const LATEST_RPC_BLOCK: BlockNumber = 2;

        fn mock_provider(
            prover_contract_code_results: &[&'static [u8]],
        ) -> Arc<dyn BlockingProvider> {
            let provider = MockProvider::new();
            for contract_code in prover_contract_code_results {
                provider
                    .push::<Bytes, Bytes>(Bytes::from_static(contract_code))
                    .unwrap();
            }
            provider
                .push::<U64, U64>(U64::from(LATEST_RPC_BLOCK))
                .unwrap();
            Arc::new(EthersProvider::new(ethers_providers::Provider::new(provider)))
        }

        fn builder(prover_contract_code_results: &[&'static [u8]]) -> WithStartChainId {
            let start_chain_provider = mock_provider(prover_contract_code_results);
            let providers = CachedMultiProvider::from_provider(CHAIN_ID, start_chain_provider);
            let chain_client =
                Box::new(PartiallySyncedClient::new(SyncStatus::new(0, LATEST_INDEXED_BLOCK)));
            let op_client_factory = Box::new(cached::Factory::default());
            WithStartChainId {
                start_chain_id: CHAIN_ID,
                chain_client,
                providers,
                op_client_factory,
            }
        }

        #[tokio::test(flavor = "multi_thread")]
        async fn prover_contract_not_deployed() {
            let builder = builder(&[b""]); // empty contract code at latest RPC block
            let res = builder.with_prover_contract_addr(Address::default()).await;
            assert!(matches!(res, Err(Error::ProverContractNotDeployed(_, _))));
        }

        #[tokio::test(flavor = "multi_thread")]
        async fn prover_contract_not_indexed() {
            // empty contract code at latest indexed block, some fake code at latest RPC block
            // (mock client is LIFO, hence the order of results)
            let builder = builder(&[b"", b"01"]);
            let res = builder
                .with_prover_contract_addr(Address::default())
                .await
                .unwrap();
            assert_eq!(res.start_exec_location, (CHAIN_ID, LATEST_RPC_BLOCK).into());
        }

        #[tokio::test(flavor = "multi_thread")]
        async fn prover_contract_indexed() {
            let builder = builder(&[b"01", b"01"]);
            let res = builder
                .with_prover_contract_addr(Address::default())
                .await
                .unwrap();
            assert_eq!(res.start_exec_location, (CHAIN_ID, LATEST_INDEXED_BLOCK).into());
        }
    }
}

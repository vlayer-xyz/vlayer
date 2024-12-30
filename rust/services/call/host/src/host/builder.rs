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
//!            ║ `with_start_chain_id` - get provider for the starting chain
//!            ║
//!            ╚>`WithStartChainProvider`   
//!               ║ `with_prover_contract_addr` - calculate start execution location,
//!               ║ (ensuring that the prover contract is deployed on that location)
//!               ║
//!               ╚>`WithStartExecLocation`
//!                  ║ `build`
//!                  ║
//!                  ╚══>`Host`

use std::{collections::HashMap, sync::Arc};

use alloy_primitives::ChainId;
use call_engine::evm::env::location::ExecutionLocation;
use provider::{
    Address, BlockNumber, BlockingProvider, CachedMultiProvider, EthersProviderFactory,
};
use risc0_zkvm::sha::Digest;
use tracing::warn;

use super::{Config, Error, Host};

pub struct New;

pub struct WithProviders {
    rpc_urls: HashMap<ChainId, String>,
    providers: CachedMultiProvider,
}

pub struct WithChainGuestId {
    rpc_urls: HashMap<ChainId, String>,
    providers: CachedMultiProvider,
    chain_guest_id: Digest,
}

pub struct WithChainClient {
    chain_client: Box<dyn chain_client::Client>,
    providers: CachedMultiProvider,
}

pub struct WithStartChainProvider {
    start_chain_provider: Arc<dyn BlockingProvider>,
    start_chain_id: ChainId,
    chain_client: Box<dyn chain_client::Client>,
    providers: CachedMultiProvider,
}

pub struct WithStartExecLocation {
    chain_client: Box<dyn chain_client::Client>,
    start_exec_location: ExecutionLocation,
    providers: CachedMultiProvider,
}

impl New {
    #[allow(clippy::unused_self)]
    #[must_use]
    pub fn with_rpc_urls(self, rpc_urls: HashMap<ChainId, String>) -> WithProviders {
        let provider_factory = EthersProviderFactory::new(rpc_urls.clone());
        let providers = CachedMultiProvider::from_factory(provider_factory);
        WithProviders {
            rpc_urls,
            providers,
        }
    }
}

impl WithProviders {
    pub fn with_chain_guest_id(self, chain_guest_id: Digest) -> WithChainGuestId {
        WithChainGuestId {
            rpc_urls: self.rpc_urls,
            providers: self.providers,
            chain_guest_id,
        }
    }
}

impl WithChainGuestId {
    pub fn with_chain_proof_url(
        self,
        chain_proof_url: &Option<String>,
    ) -> Result<WithChainClient, Error> {
        let WithChainGuestId {
            rpc_urls,
            providers,
            chain_guest_id,
        } = self;
        let chain_client: Box<dyn chain_client::Client> = match chain_proof_url.as_ref() {
            Some(url) => Box::new(chain_client::RpcClient::new(url)),
            None => {
                warn!("Chain proof sever URL not provided. Running with mock server");
                let provider_factory = EthersProviderFactory::new(rpc_urls);
                let providers = CachedMultiProvider::from_factory(provider_factory);
                Box::new(chain_client::FakeClient::new(providers, chain_guest_id))
            }
        };
        Ok(WithChainClient {
            chain_client,
            providers,
        })
    }
}

impl WithChainClient {
    pub fn with_start_chain_id(
        self,
        start_chain_id: ChainId,
    ) -> Result<WithStartChainProvider, Error> {
        let WithChainClient {
            chain_client,
            providers,
        } = self;
        let start_chain_provider = providers.get(start_chain_id)?;
        Ok(WithStartChainProvider {
            start_chain_provider,
            start_chain_id,
            providers,
            chain_client,
        })
    }
}

impl WithStartChainProvider {
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
        let WithStartChainProvider {
            start_chain_provider,
            start_chain_id,
            chain_client,
            providers,
        } = self;

        let prover_contract_deployed =
            check_prover_contract(&start_chain_provider, prover_contract_addr);

        let latest_rpc_block = start_chain_provider.get_latest_block_number()?;
        if !prover_contract_deployed(latest_rpc_block)? {
            return Err(Error::ProverContractNotDeployed);
        }

        let latest_indexed_block = chain_client
            .get_sync_status(start_chain_id)
            .await?
            .last_block;
        let start_block_number = if prover_contract_deployed(latest_indexed_block)? {
            latest_indexed_block
        } else {
            latest_rpc_block
        };
        let start_exec_location = (start_chain_id, start_block_number).into();

        Ok(WithStartExecLocation {
            chain_client,
            start_exec_location,
            providers,
        })
    }
}

impl WithStartExecLocation {
    #[must_use]
    pub fn build(self, config: Config) -> Host {
        let WithStartExecLocation {
            chain_client,
            start_exec_location,
            providers,
        } = self;
        Host::new(providers, start_exec_location, chain_client, config)
    }
}

fn check_prover_contract(
    provider: &Arc<dyn BlockingProvider>,
    address: Address,
) -> impl Fn(BlockNumber) -> Result<bool, Error> + '_ {
    move |block_num| Ok(!provider.get_code(address, block_num)?.is_empty())
}

#[cfg(test)]
mod tests {
    use super::*;

    mod start_exec_location {
        use chain_common::{GetSyncStatus, SyncStatus};
        use ethers_core::types::{Bytes, U64};
        use ethers_providers::MockProvider;
        use mock_chain_server::ChainProofServerMock;
        use provider::{EthersProvider, NullProviderFactory};

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

        async fn mock_chain_client() -> Box<dyn chain_client::Client> {
            let mut chain_server = ChainProofServerMock::start().await;
            chain_server
                .mock_sync_status()
                .with_params(GetSyncStatus::new(CHAIN_ID), true)
                .with_result(SyncStatus::new(0, LATEST_INDEXED_BLOCK))
                .add()
                .await;
            Box::new(chain_server.into_client())
        }

        async fn builder(prover_contract_code_results: &[&'static [u8]]) -> WithStartChainProvider {
            let start_chain_provider = mock_provider(prover_contract_code_results);
            let chain_client = mock_chain_client().await;
            let providers = CachedMultiProvider::from_factory(NullProviderFactory);
            WithStartChainProvider {
                start_chain_provider,
                start_chain_id: CHAIN_ID,
                chain_client,
                providers,
            }
        }

        #[tokio::test(flavor = "multi_thread")]
        async fn prover_contract_not_deployed() {
            let builder = builder(&[b""]).await; // empty contract code at latest RPC block
            let res = builder.with_prover_contract_addr(Address::default()).await;
            assert!(matches!(res, Err(Error::ProverContractNotDeployed)));
        }

        #[tokio::test(flavor = "multi_thread")]
        async fn prover_contract_not_indexed() {
            // empty contract code at latest indexed block, some fake code at latest RPC block
            // (mock client is LIFO, hence the order of results)
            let builder = builder(&[b"", b"01"]).await;
            let res = builder
                .with_prover_contract_addr(Address::default())
                .await
                .unwrap();
            assert_eq!(res.start_exec_location, (CHAIN_ID, LATEST_RPC_BLOCK).into());
        }

        #[tokio::test(flavor = "multi_thread")]
        async fn prover_contract_indexed() {
            let builder = builder(&[b"01", b"01"]).await;
            let res = builder
                .with_prover_contract_addr(Address::default())
                .await
                .unwrap();
            assert_eq!(res.start_exec_location, (CHAIN_ID, LATEST_INDEXED_BLOCK).into());
        }
    }
}

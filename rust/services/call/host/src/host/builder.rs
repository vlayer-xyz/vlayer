//! Host builder is a state machine with five linear stages.
//! Most stage transitions are fallible, some are asynchronous.
//!
//! `New`
//!  ║ `with_rpc_urls` - create `CachedMultiProvider` from URLs
//!  ║
//!  ╚>`WithProviders`
//!     ║ `with_start_chain_id` - get provider for the starting chain
//!     ║
//!     ╚>`WithStartChainProvider`
//!        ║ `with_chain_proof_url` - create chain proof client from the given URL
//!        ║ or mock it using RPC provider for the starting chain
//!        ║
//!        ╚>`WithChainClient`
//!           ║ `with_prover_contract_addr` - calculate start execution location,
//!           ║ (ensuring that the prover contract is deployed on that location)
//!           ║
//!           ╚>`WithStartExecLocation`
//!              ║ `build`
//!              ║
//!              ╚══>`Host`

use std::{collections::HashMap, sync::Arc};

use alloy_primitives::ChainId;
use call_engine::evm::env::location::ExecutionLocation;
use mock_chain_server::ChainProofServerMock;
use provider::{
    Address, BlockNumber, BlockTag, BlockingProvider, CachedMultiProvider, EthersProviderFactory,
};
use tracing::warn;

use super::{Config, Error, Host};

pub struct New;

pub struct WithProviders {
    providers: CachedMultiProvider,
}

pub struct WithStartChainProvider {
    start_chain_provider: Arc<dyn BlockingProvider>,
    start_chain_id: ChainId,
    providers: CachedMultiProvider,
}

pub struct WithChainClient {
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
        let provider_factory = EthersProviderFactory::new(rpc_urls);
        let providers = CachedMultiProvider::from_factory(provider_factory);
        WithProviders { providers }
    }
}

impl WithProviders {
    pub fn with_start_chain_id(
        self,
        start_chain_id: ChainId,
    ) -> Result<WithStartChainProvider, Error> {
        let providers = self.providers;
        let start_chain_provider = providers.get(start_chain_id)?;
        Ok(WithStartChainProvider {
            start_chain_provider,
            start_chain_id,
            providers,
        })
    }
}

impl WithStartChainProvider {
    pub async fn with_chain_proof_url(
        self,
        chain_proof_url: &Option<String>,
    ) -> Result<WithChainClient, Error> {
        let WithStartChainProvider {
            start_chain_provider,
            start_chain_id,
            providers,
        } = self;
        let chain_client = match chain_proof_url.as_ref() {
            Some(url) => Box::new(chain_client::RpcClient::new(url)),
            None => {
                warn!("Chain proof sever URL not provided. Running with mock server");
                mock_chain_client(&start_chain_provider, start_chain_id).await?
            }
        };
        Ok(WithChainClient {
            start_chain_provider,
            start_chain_id,
            chain_client,
            providers,
        })
    }
}

impl WithChainClient {
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
        let WithChainClient {
            start_chain_provider,
            chain_client,
            start_chain_id,
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

async fn mock_chain_client(
    start_chain_provider: &Arc<dyn BlockingProvider>,
    chain_id: ChainId,
) -> Result<Box<dyn chain_client::Client>, Error> {
    let latest_block = start_chain_provider
        .get_block_header(BlockTag::Latest)?
        .ok_or_else(|| Error::Provider("latest block not found".to_string()))?;
    let mut chain_proof_server = ChainProofServerMock::start().await;
    chain_proof_server
        .mock_single_block(chain_id, latest_block)
        .await;
    let chain_client = Box::new(chain_proof_server.into_client());
    Ok(chain_client)
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
        use alloy_primitives::{ChainId, U64};
        use chain_common::{GetSyncStatus, SyncStatus};
        use ethers_core::types::Bytes;
        use ethers_providers::MockProvider;
        use mock_chain_server::ChainProofServerMock;
        use provider::{Address, CachedMultiProvider, EthersProvider, NullProviderFactory};

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

        async fn builder(prover_contract_code_results: &[&'static [u8]]) -> WithChainClient {
            let start_chain_provider = mock_provider(prover_contract_code_results);
            let chain_client = mock_chain_client().await;
            let providers = CachedMultiProvider::from_factory(NullProviderFactory);
            WithChainClient {
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

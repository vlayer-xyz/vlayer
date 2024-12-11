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

pub struct WithProviders(CachedMultiProvider);

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
        WithProviders(providers)
    }
}

impl WithProviders {
    pub fn with_start_chain_id(
        self,
        start_chain_id: ChainId,
    ) -> Result<WithStartChainProvider, Error> {
        let providers = self.0;
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

        let latest_rpc_block = start_chain_provider
            .get_block_header(BlockTag::Latest)?
            .ok_or_else(|| Error::Provider("latest block not found".to_string()))?
            .number();
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

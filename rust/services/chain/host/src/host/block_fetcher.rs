use std::result;

use common::Hashable;
use derivative::Derivative;
use ethers::{
    middleware::Middleware,
    providers::{Http, JsonRpcClient, Provider},
    types::BlockNumber as BlockTag,
};
use futures::{StreamExt, TryStreamExt, stream};
use provider::{BlockNumber, EvmBlockHeader, to_eth_block_header};
use thiserror::Error;
use tracing::{debug, instrument};
use u64_range::Range;
use url::ParseError;

const MAX_CONCURRENT_RPC_REQUESTS: usize = 10;

pub struct BlockFetcher<P>
where
    P: JsonRpcClient,
{
    provider: Provider<P>,
}

impl BlockFetcher<Http> {
    pub fn new(rpc_url: String) -> Result<Self> {
        let provider = Provider::<Http>::try_from(rpc_url)?;
        Ok(BlockFetcher { provider })
    }
}

type Result<T> = result::Result<T, BlockFetcherError>;

#[derive(Error, Debug, Derivative)]
#[derivative(PartialEq, Eq)]
pub enum BlockFetcherError {
    #[error("Provider: {0}")]
    Provider(
        #[from]
        #[derivative(PartialEq = "ignore")]
        ethers::providers::ProviderError,
    ),
    #[error("BlockNotFound: {0}")]
    BlockNotFound(BlockTag),
    #[error("Block conversion error: {0}")]
    BlockConversion(String),
    #[error("Parse error: {0}")]
    ParseError(#[from] ParseError),
}

impl<P> BlockFetcher<P>
where
    P: JsonRpcClient,
{
    pub const fn from_provider(provider: Provider<P>) -> Self {
        BlockFetcher { provider }
    }

    #[instrument(skip(self))]
    pub async fn get_blocks_range(&self, range: Range) -> Result<Vec<Box<dyn EvmBlockHeader>>> {
        stream::iter(range)
            .map(|n| self.get_block(n.into()))
            .buffered(MAX_CONCURRENT_RPC_REQUESTS)
            .try_collect()
            .await
    }

    #[instrument(skip(self))]
    pub async fn get_block(&self, number: BlockTag) -> Result<Box<dyn EvmBlockHeader>> {
        debug!("Fetching block {number}");
        let ethers_block = self
            .provider
            .get_block(number)
            .await?
            .ok_or(BlockFetcherError::BlockNotFound(number))?;
        let block = to_eth_block_header(ethers_block)
            .map_err(|e| BlockFetcherError::BlockConversion(e.to_string()))?;
        debug!("Fetched block {} with hash {}", block.number(), block.hash_slow());
        Ok(Box::new(block))
    }

    #[instrument(skip(self))]
    pub async fn get_latest_block_number(&self) -> Result<BlockNumber> {
        debug!("Getting latest block number");
        Ok(self.provider.get_block_number().await.map(|block_num| {
            debug!("Latest block number: {block_num}");
            block_num.as_u64()
        })?)
    }
}

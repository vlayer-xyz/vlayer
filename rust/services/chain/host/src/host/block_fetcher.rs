use std::result;

use derivative::Derivative;
use ethers::{
    middleware::Middleware,
    providers::{Http, JsonRpcClient, Provider},
    types::BlockNumber as BlockTag,
};
use futures::future::join_all;
use provider::{to_eth_block_header, EvmBlockHeader};
use thiserror::Error;
use tracing::{info, instrument};
use u64_range::Range;
use url::ParseError;

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
    Provider(String),
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
        let blocks = join_all(range.into_iter().map(|n| self.get_block(n.into()))).await;
        blocks.into_iter().collect()
    }

    #[instrument(skip(self))]
    pub async fn get_block(&self, number: BlockTag) -> Result<Box<dyn EvmBlockHeader>> {
        info!("Fetching block {}", number);
        let ethers_block = self
            .provider
            .get_block(number)
            .await
            .map_err(|err| BlockFetcherError::Provider(err.to_string()))?
            .ok_or(BlockFetcherError::BlockNotFound(number))?;
        let block = to_eth_block_header(ethers_block)
            .map_err(|e| BlockFetcherError::BlockConversion(e.to_string()))?;
        info!("Fetched block {}", block.number());
        Ok(Box::new(block))
    }
}

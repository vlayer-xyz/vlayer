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

pub struct BlockFetcher<P>
where
    P: JsonRpcClient,
{
    pub(crate) provider: Provider<P>,
}

impl BlockFetcher<Http> {
    pub fn new(rpc_url: String) -> Self {
        let provider =
            Provider::<Http>::try_from(rpc_url).expect("could not instantiate HTTP Provider");
        BlockFetcher { provider }
    }
}

#[derive(Error, Debug, Derivative)]
#[derivative(PartialEq, Eq)]
pub enum BlockFetcherError {
    #[error("Provider: {0}")]
    Provider(String),
    #[error("BlockNotFound: {0}")]
    BlockNotFound(BlockTag),
    #[error("Block conversion error: {0}")]
    BlockConversion(String),
}

impl<P> BlockFetcher<P>
where
    P: JsonRpcClient,
{
    pub const fn from_provider(provider: Provider<P>) -> Self {
        BlockFetcher { provider }
    }

    #[instrument(skip(self))]
    pub async fn get_blocks_range(
        &self,
        range: Range,
    ) -> Result<Vec<Box<dyn EvmBlockHeader>>, BlockFetcherError> {
        let blocks = join_all(range.into_iter().map(|n| self.get_block(n.into()))).await;
        blocks.into_iter().collect()
    }

    #[instrument(skip(self))]
    pub async fn get_block(
        &self,
        number: BlockTag,
    ) -> Result<Box<dyn EvmBlockHeader>, BlockFetcherError> {
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

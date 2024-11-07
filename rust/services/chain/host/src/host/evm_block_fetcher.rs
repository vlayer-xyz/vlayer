use ethers::{
    middleware::Middleware,
    providers::{Http, JsonRpcClient, Provider},
    types::BlockNumber as BlockTag,
};
use futures::future::join_all;
use provider::{to_eth_block_header, EvmBlockHeader};
use tracing::{info, instrument};
use u64_range::Range;

use super::HostError;

pub struct EvmBlockFetcher<P>
where
    P: JsonRpcClient,
{
    pub(crate) provider: Provider<P>,
}

impl EvmBlockFetcher<Http> {
    pub fn new(rpc_url: String) -> Self {
        let provider =
            Provider::<Http>::try_from(rpc_url).expect("could not instantiate HTTP Provider");
        EvmBlockFetcher { provider }
    }
}

impl<P> EvmBlockFetcher<P>
where
    P: JsonRpcClient,
{
    pub fn from_provider(provider: Provider<P>) -> Self {
        EvmBlockFetcher { provider }
    }

    #[instrument(skip(self))]
    pub async fn get_blocks_range(
        &self,
        range: Range,
    ) -> Result<Vec<Box<dyn EvmBlockHeader>>, HostError> {
        let blocks = join_all(range.into_iter().map(|n| self.get_block(n.into()))).await;
        blocks.into_iter().collect()
    }

    #[instrument(skip(self))]
    pub async fn get_block(&self, number: BlockTag) -> Result<Box<dyn EvmBlockHeader>, HostError> {
        info!("Fetching block {}", number);
        let ethers_block = self
            .provider
            .get_block(number)
            .await
            .map_err(|err| HostError::Provider(err.to_string()))?
            .ok_or(HostError::BlockNotFound(number))?;
        let block = to_eth_block_header(ethers_block)
            .map_err(|e| HostError::BlockConversion(e.to_string()))?;
        info!("Fetched block {}", block.number());
        Ok(Box::new(block))
    }
}

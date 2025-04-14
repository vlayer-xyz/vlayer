use std::sync::Arc;

use anyhow::{Result, anyhow};
use async_trait::async_trait;
use block_header::EvmBlockHeader;
use ethers_core::types::BlockNumber as BlockTag;
use futures::{TryStreamExt, stream::FuturesOrdered};

use crate::BlockingProvider;

#[async_trait]
pub trait BlockingProviderExt: BlockingProvider + 'static {
    async fn get_block_headers(
        self: Arc<Self>,
        blocks: Vec<BlockTag>,
    ) -> Result<Vec<Box<dyn EvmBlockHeader>>> {
        blocks
            .into_iter()
            .map(|block| {
                let provider = self.clone();
                async move {
                    tokio::task::spawn_blocking(move || provider.get_block_header(block))
                        .await??
                        .ok_or_else(|| anyhow!("Block {block} not found"))
                }
            })
            .collect::<FuturesOrdered<_>>()
            .try_collect::<Vec<_>>()
            .await
    }
}

#[async_trait]
impl<P: BlockingProvider + 'static + ?Sized> BlockingProviderExt for P {}

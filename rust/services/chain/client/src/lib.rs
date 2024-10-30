use std::collections::HashMap;

use alloy_primitives::ChainId;
use async_trait::async_trait;
use chain_server::server::ChainProof as RpcChainProof;
use chain_types::ChainProof;
use futures::stream::{self, StreamExt, TryStreamExt};
use mpt::ParseNodeError;
use parking_lot::RwLock;
use provider::BlockNumber;
use serde_json::json;
use server_utils::{RpcClient, RpcError};
use thiserror::Error;
use tracing::info;

#[cfg(test)]
mod tests;

#[async_trait]
pub trait ChainProofClient {
    async fn fetch_chain_proof(
        &self,
        chain_id: ChainId,
        block_numbers: Vec<BlockNumber>,
    ) -> Result<ChainProof, ChainProofClientError>;

    async fn get_chain_proofs(
        &self,
        blocks_by_chain: HashMap<ChainId, Vec<BlockNumber>>,
    ) -> Result<HashMap<ChainId, ChainProof>, ChainProofClientError> {
        stream::iter(blocks_by_chain)
            .then(|(chain_id, block_numbers)| async move {
                Ok((chain_id, self.fetch_chain_proof(chain_id, block_numbers).await?))
            })
            .try_collect()
            .await
    }
}

#[derive(Debug, Error)]
pub enum ChainProofClientError {
    #[error("RPC error: {0}")]
    Rpc(#[from] RpcError),
    #[error("JSON parse error: {0}")]
    Json(#[from] serde_json::Error),
    #[error("MPT node parse error: {0}")]
    MptNode(#[from] ParseNodeError),
    #[error("Proof not found in cache. chain_id={chain_id} block_numbers={block_numbers:?}")]
    CacheMiss {
        chain_id: ChainId,
        block_numbers: Vec<BlockNumber>,
    },
}

pub struct RpcChainProofClient {
    rpc_client: RpcClient,
}

impl RpcChainProofClient {
    pub fn new(base_url: impl AsRef<str>) -> Self {
        let rpc_client = RpcClient::new(base_url.as_ref(), "v_chain");
        Self { rpc_client }
    }
}

#[async_trait]
impl ChainProofClient for RpcChainProofClient {
    async fn fetch_chain_proof(
        &self,
        chain_id: ChainId,
        block_numbers: Vec<BlockNumber>,
    ) -> Result<ChainProof, ChainProofClientError> {
        info!(
            "Fetching chain proof for chain_id: {}, block_numbers.len(): {}",
            chain_id,
            block_numbers.len()
        );

        let params = json!({
            "chain_id": chain_id,
            "block_numbers": block_numbers.clone(),
        });

        let result_value = self
            .rpc_client
            .call(&params)
            .await
            .map_err(ChainProofClientError::from)?;

        let rpc_chain_proof: RpcChainProof = serde_json::from_value(result_value)?;
        let chain_proof = rpc_chain_proof.try_into()?;

        Ok(chain_proof)
    }
}

type ChainProofCache = HashMap<ChainId, (Vec<BlockNumber>, ChainProof)>;

pub struct CachingChainProofClient<T: ChainProofClient> {
    inner: T,
    cache: RwLock<ChainProofCache>,
}

impl<T: ChainProofClient> CachingChainProofClient<T> {
    #[must_use]
    pub fn new(inner: T) -> Self {
        Self {
            inner,
            cache: RwLock::new(Default::default()),
        }
    }

    pub fn into_cache(self) -> ChainProofCache {
        self.cache.into_inner()
    }

    pub fn into_cached_client(self) -> CachedChainProofClient {
        CachedChainProofClient::new(self.into_cache())
    }
}

#[async_trait]
impl<T: ChainProofClient + Send + Sync> ChainProofClient for CachingChainProofClient<T> {
    async fn fetch_chain_proof(
        &self,
        chain_id: ChainId,
        block_numbers: Vec<BlockNumber>,
    ) -> Result<ChainProof, ChainProofClientError> {
        let proof = self
            .inner
            .fetch_chain_proof(chain_id, block_numbers.clone())
            .await?;
        self.cache
            .write()
            .insert(chain_id, (block_numbers, proof.clone()));
        Ok(proof)
    }
}

pub struct CachedChainProofClient {
    cache: ChainProofCache,
}

impl CachedChainProofClient {
    pub fn new(cache: ChainProofCache) -> Self {
        Self { cache }
    }
}

#[async_trait]
impl ChainProofClient for CachedChainProofClient {
    async fn fetch_chain_proof(
        &self,
        chain_id: ChainId,
        block_numbers: Vec<BlockNumber>,
    ) -> Result<ChainProof, ChainProofClientError> {
        match self.cache.get(&chain_id) {
            Some((blocks, proof)) if blocks == &block_numbers => Ok(proof.clone()),
            _ => Err(ChainProofClientError::CacheMiss {
                chain_id,
                block_numbers,
            }),
        }
    }
}

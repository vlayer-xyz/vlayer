use std::collections::HashMap;

use alloy_primitives::ChainId;
use async_trait::async_trait;
use chain_server::server::ChainProof as RpcChainProof;
use chain_shared::ChainProof;
use futures::stream::{self, StreamExt, TryStreamExt};
use mpt::ParseNodeError;
use parking_lot::RwLock;
use provider::BlockNumber;
use serde_json::json;
use server_utils::{RpcClient as RawRpcClient, RpcError};
use thiserror::Error;
use tracing::info;

#[cfg(test)]
mod tests;

#[async_trait]
pub trait Client: Send + Sync {
    async fn get_chain_proof(
        &self,
        chain_id: ChainId,
        block_numbers: Vec<BlockNumber>,
    ) -> Result<ChainProof, Error>;

    async fn get_chain_proofs(
        &self,
        blocks_by_chain: HashMap<ChainId, Vec<BlockNumber>>,
    ) -> Result<HashMap<ChainId, ChainProof>, Error> {
        stream::iter(blocks_by_chain)
            .then(|(chain_id, block_numbers)| async move {
                Ok((chain_id, self.get_chain_proof(chain_id, block_numbers).await?))
            })
            .try_collect()
            .await
    }
}

#[derive(Debug, Error)]
pub enum Error {
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

/// `Client` implementation which fetches proofs from server via JSON RPC.
pub struct RpcClient {
    rpc_client: RawRpcClient,
}

impl RpcClient {
    pub fn new(base_url: impl AsRef<str>) -> Self {
        let rpc_client = RawRpcClient::new(base_url.as_ref(), "v_chain");
        Self { rpc_client }
    }
}

#[async_trait]
impl Client for RpcClient {
    async fn get_chain_proof(
        &self,
        chain_id: ChainId,
        block_numbers: Vec<BlockNumber>,
    ) -> Result<ChainProof, Error> {
        info!(
            "Fetching chain proof for chain_id: {}, block_numbers.len(): {}",
            chain_id,
            block_numbers.len()
        );

        let params = json!({
            "chain_id": chain_id,
            "block_numbers": block_numbers.clone(),
        });

        let result_value = self.rpc_client.call(&params).await.map_err(Error::from)?;

        let rpc_chain_proof: RpcChainProof = serde_json::from_value(result_value)?;
        let chain_proof = rpc_chain_proof.try_into()?;

        Ok(chain_proof)
    }
}

type ChainProofCache = HashMap<ChainId, (Vec<BlockNumber>, ChainProof)>;

/// `Client` implementation which wraps around another client (e.g. `RpcClient`) and stores
/// all obtained proofs in a `ChainProofCache`. The cache can be dumped via `into_cache`
/// and used to create an instance of `CachedClient`.
pub struct RecordingClient<T: Client> {
    inner: T,
    cache: RwLock<ChainProofCache>,
}

impl<T: Client> RecordingClient<T> {
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

    pub fn into_cached_client(self) -> CachedClient {
        CachedClient::new(self.into_cache())
    }
}

#[async_trait]
impl<T: Client> Client for RecordingClient<T> {
    async fn get_chain_proof(
        &self,
        chain_id: ChainId,
        block_numbers: Vec<BlockNumber>,
    ) -> Result<ChainProof, Error> {
        let proof = self
            .inner
            .get_chain_proof(chain_id, block_numbers.clone())
            .await?;
        self.cache
            .write()
            .insert(chain_id, (block_numbers, proof.clone()));
        Ok(proof)
    }
}

/// `Client` implementation which only reads proofs from cache.
pub struct CachedClient {
    cache: ChainProofCache,
}

impl CachedClient {
    pub fn new(cache: ChainProofCache) -> Self {
        Self { cache }
    }
}

#[async_trait]
impl Client for CachedClient {
    async fn get_chain_proof(
        &self,
        chain_id: ChainId,
        block_numbers: Vec<BlockNumber>,
    ) -> Result<ChainProof, Error> {
        match self.cache.get(&chain_id) {
            Some((blocks, proof)) if blocks == &block_numbers => Ok(proof.clone()),
            _ => Err(Error::CacheMiss {
                chain_id,
                block_numbers,
            }),
        }
    }
}

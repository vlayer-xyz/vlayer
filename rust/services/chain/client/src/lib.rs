use std::{collections::HashMap, fmt::Display, sync::Arc, time::Duration};

use alloy_primitives::{BlockNumber, ChainId};
use async_trait::async_trait;
use chain_common::{ChainProof, SyncStatus};
use derive_new::new;
use parking_lot::RwLock;
use thiserror::Error;

#[cfg(feature = "fake")]
mod fake;
#[cfg(feature = "rpc")]
mod rpc;

#[cfg(feature = "fake")]
pub use fake::FakeClient;
#[cfg(feature = "fake")]
pub use fake::PartiallySyncedClient;
#[cfg(feature = "rpc")]
pub use rpc::{Config as ChainClientConfig, RpcClient};

#[cfg(test)]
mod tests;

#[async_trait]
pub trait Client: Send + Sync + 'static {
    async fn get_chain_proof(
        &self,
        chain_id: ChainId,
        block_numbers: Vec<BlockNumber>,
    ) -> Result<ChainProof, Error>;

    async fn get_sync_status(&self, chain_id: ChainId) -> Result<SyncStatus, Error>;
}

#[derive(Debug, Error)]
pub enum Error {
    #[cfg(feature = "rpc")]
    #[error("RPC error: {0}")]
    Rpc(#[from] server_utils::rpc::Error),
    #[cfg(feature = "rpc")]
    #[error("JSON parse error: {0}")]
    Json(#[from] serde_json::Error),
    #[cfg(feature = "rpc")]
    #[error("Proof parse error: {0}")]
    ParseProof(#[from] chain_common::ParseProofError),
    #[error("Proof not found in cache. chain_id={chain_id} block_numbers={block_numbers:?}")]
    CacheMiss {
        chain_id: ChainId,
        block_numbers: Vec<BlockNumber>,
    },
    #[error("Chain {0} not supported")]
    UnsupportedChain(ChainId),
    #[error(
        "Waiting for chain proof timed out. chain_id={chain_id} block_numbers={block_numbers:?} timeout={timeout:?} sync_status={sync_status:?}"
    )]
    Timeout {
        chain_id: ChainId,
        block_numbers: Vec<BlockNumber>,
        timeout: Duration,
        sync_status: SyncStatus,
    },
    #[error(
        "Chain {chain_id} is too far behind. Behind {behind} blocks. Block numbers: {block_numbers:?}. Sync status: {sync_status:?}"
    )]
    TooFarBehind {
        chain_id: ChainId,
        behind: u64,
        block_numbers: Vec<BlockNumber>,
        sync_status: SyncStatus,
    },
    #[error("{0}")]
    Other(String),
}

impl Error {
    pub fn other(msg: impl Display) -> Self {
        Self::Other(msg.to_string())
    }
}

pub type ChainProofCache = HashMap<ChainId, (Vec<BlockNumber>, ChainProof)>;

/// `Client` implementation which wraps around another client (e.g. `RpcClient`) and stores all obtained proofs in a `ChainProofCache`.
///
/// The cache can be dumped via `into_cache` and used to create an instance of `CachedClient`.
#[derive(Clone)]
pub struct RecordingClient {
    inner: Arc<dyn Client>,
    cache: Arc<RwLock<ChainProofCache>>,
}

impl RecordingClient {
    #[must_use]
    pub fn new(inner: Box<dyn Client>) -> Self {
        Self {
            inner: inner.into(),
            cache: Arc::new(RwLock::new(Default::default())),
        }
    }

    #[allow(clippy::expect_used)]
    pub fn into_cache(self) -> ChainProofCache {
        let cache =
            Arc::try_unwrap(self.cache).expect("Trying to access cache while it's still in use");
        cache.into_inner()
    }

    pub fn into_cached_client(self) -> CachedClient {
        CachedClient::new(self.into_cache())
    }
}

#[async_trait]
impl Client for RecordingClient {
    async fn get_chain_proof(
        &self,
        chain_id: ChainId,
        mut block_numbers: Vec<BlockNumber>,
    ) -> Result<ChainProof, Error> {
        // Block numbers should be sorted, because the vector is compared
        block_numbers.sort();
        let proof = self
            .inner
            .get_chain_proof(chain_id, block_numbers.clone())
            .await?;
        self.cache
            .write()
            .insert(chain_id, (block_numbers, proof.clone()));
        Ok(proof)
    }

    async fn get_sync_status(&self, chain_id: ChainId) -> Result<SyncStatus, Error> {
        // sync status requests are not cached, as they won't be used in guest code
        self.inner.get_sync_status(chain_id).await
    }
}

/// `Client` implementation which only reads proofs from cache.
#[derive(new)]
pub struct CachedClient {
    cache: ChainProofCache,
}

#[async_trait]
impl Client for CachedClient {
    async fn get_chain_proof(
        &self,
        chain_id: ChainId,
        mut block_numbers: Vec<BlockNumber>,
    ) -> Result<ChainProof, Error> {
        block_numbers.sort();
        match self.cache.get(&chain_id) {
            Some((blocks, proof)) if blocks == &block_numbers => Ok(proof.clone()),
            _ => Err(Error::CacheMiss {
                chain_id,
                block_numbers,
            }),
        }
    }

    #[allow(clippy::unwrap_used)]
    async fn get_sync_status(&self, chain_id: ChainId) -> Result<SyncStatus, Error> {
        match self.cache.get(&chain_id) {
            Some((blocks, _)) if !blocks.is_empty() => {
                Ok(SyncStatus::new(*blocks.first().unwrap(), *blocks.last().unwrap()))
            }
            _ => Ok(SyncStatus::default()),
        }
    }
}

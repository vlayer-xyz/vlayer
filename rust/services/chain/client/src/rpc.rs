use std::time::Duration;

use alloy_primitives::{BlockNumber, ChainId};
use async_trait::async_trait;
use chain_common::{ChainProof, GetChainProof, GetSyncStatus, RpcChainProof, SyncStatus};
use derive_new::new;
use serde::{Deserialize, Serialize};
use server_utils::rpc::Client as RawRpcClient;
use tracing::info;

use crate::{Client, Error};

#[derive(new, Serialize, Deserialize, Clone, Debug)]
pub struct Config {
    pub url: String,
    pub poll_interval: Duration,
    pub timeout: Duration,
}

/// `Client` implementation which fetches proofs from server via JSON RPC.
pub struct RpcClient {
    rpc_client: RawRpcClient,
    poll_interval: Duration,
    timeout: Duration,
}

impl RpcClient {
    pub fn new(config: &Config) -> Self {
        let rpc_client = RawRpcClient::new(&config.url);
        Self {
            rpc_client,
            poll_interval: config.poll_interval,
            timeout: config.timeout,
        }
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
            "Fetching chain proof for chain_id: {chain_id}, block_numbers.len(): {}",
            block_numbers.len()
        );

        let params = GetChainProof::new(chain_id, block_numbers.clone());
        let result_value = self.rpc_client.call(params).await.map_err(Error::from)?;

        let rpc_chain_proof: RpcChainProof = serde_json::from_value(result_value)?;
        let chain_proof = rpc_chain_proof.try_into()?;

        Ok(chain_proof)
    }

    async fn get_sync_status(&self, chain_id: ChainId) -> Result<SyncStatus, Error> {
        info!("Getting sync status for chain_id: {chain_id}");

        let params = GetSyncStatus::new(chain_id);
        let result_value = self.rpc_client.call(params).await.map_err(Error::from)?;
        let sync_status: SyncStatus = serde_json::from_value(result_value)?;

        info!("Sync status for chain {chain_id}: {sync_status:?}");
        Ok(sync_status)
    }
}

use std::{cmp::max, time::Duration};

use alloy_primitives::{BlockNumber, ChainId};
use async_trait::async_trait;
use chain_common::{ChainProof, GetChainProof, GetSyncStatus, RpcChainProof, SyncStatus};
use derive_new::new;
use serde::{Deserialize, Serialize};
use server_utils::rpc::Client as RawRpcClient;
use tracing::{error, info};

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

    async fn await_synced(
        &self,
        chain_id: ChainId,
        block_numbers: &[BlockNumber],
    ) -> Result<(), Error> {
        let start = tokio::time::Instant::now();
        loop {
            let sync_status = self.get_sync_status(chain_id).await?;
            info!(chain_id, ?sync_status, "Sync status");
            let lag = blocks_behind(&sync_status, block_numbers);
            info!(chain_id, ?lag, "Sync lag");
            if lag == 0 {
                break;
            } else if lag > LAG_TOLERANCE {
                error!(chain_id, ?lag, "Chain is too far behind");
                return Err(Error::TooFarBehind {
                    chain_id,
                    behind: lag,
                    block_numbers: block_numbers.to_vec(),
                    sync_status,
                });
            } else if start.elapsed() > self.timeout {
                error!(chain_id, ?lag, "Timeout while waiting for chain proof");
                return Err(Error::Timeout {
                    chain_id,
                    block_numbers: block_numbers.to_vec(),
                    timeout: self.timeout,
                    sync_status,
                });
            }
            info!(chain_id, ?lag, "Waiting for chain proof");
            tokio::time::sleep(self.poll_interval).await;
        }
        Ok(())
    }
}

// If out-of-sync state is within tolerance - we actively wait up to timeout
// Otherwise - give up immediately
const LAG_TOLERANCE: u64 = 100;

#[async_trait]
impl Client for RpcClient {
    async fn get_chain_proof(
        &self,
        chain_id: ChainId,
        block_numbers: Vec<BlockNumber>,
    ) -> Result<ChainProof, Error> {
        info!(chain_id, block_numbers = ?block_numbers, "Getting chain proof");
        self.await_synced(chain_id, &block_numbers).await?;
        info!(chain_id, block_numbers = ?block_numbers, "fetching chain proof");

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

#[allow(clippy::unwrap_used)]
fn blocks_behind(sync_status: &SyncStatus, block_numbers: &[BlockNumber]) -> u64 {
    assert!(!block_numbers.is_empty(), "block_numbers cannot be empty");
    let first_block_number = block_numbers.iter().min().unwrap();
    let last_block_number = block_numbers.iter().max().unwrap();
    let backprop_behind = sync_status.first_block.saturating_sub(*first_block_number);
    let head_behind = last_block_number.saturating_sub(sync_status.last_block);
    max(backprop_behind, head_behind)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn head_behind() {
        assert_eq!(blocks_behind(&SyncStatus::new(0, 1), &[2]), 1);
    }

    #[test]
    fn tail_behind() {
        assert_eq!(blocks_behind(&SyncStatus::new(1, 1), &[0]), 1);
    }

    #[test]
    fn just_synced() {
        assert_eq!(blocks_behind(&SyncStatus::new(0, 1), &[0, 1]), 0);
    }

    #[test]
    fn success() {
        assert_eq!(blocks_behind(&SyncStatus::new(0, 10), &[1]), 0);
    }
}

use alloy_primitives::{BlockNumber, ChainId};
use async_trait::async_trait;
use chain_common::{ChainProof, RpcChainProof};
use derive_new::new;
use serde::Serialize;
use server_utils::rpc::{Client as RawRpcClient, Method};
use tracing::info;

use crate::{Client, Error};

/// `Client` implementation which fetches proofs from server via JSON RPC.
pub struct RpcClient {
    rpc_client: RawRpcClient,
}

impl RpcClient {
    pub fn new(base_url: impl AsRef<str>) -> Self {
        let rpc_client = RawRpcClient::new(base_url.as_ref());
        Self { rpc_client }
    }
}

#[derive(new, Serialize)]
struct GetChainProof {
    chain_id: ChainId,
    block_numbers: Vec<BlockNumber>,
}

impl Method for GetChainProof {
    const METHOD_NAME: &str = "v_chain";
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

        let params = GetChainProof::new(chain_id, block_numbers.clone());
        let result_value = self.rpc_client.call(params).await.map_err(Error::from)?;

        let rpc_chain_proof: RpcChainProof = serde_json::from_value(result_value)?;
        let chain_proof = rpc_chain_proof.try_into()?;

        Ok(chain_proof)
    }
}

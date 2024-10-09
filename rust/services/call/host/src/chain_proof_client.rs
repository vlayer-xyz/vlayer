use std::collections::{HashMap, HashSet};

use alloy_primitives::ChainId;
use axum_jrpc::{Id, JsonRpcAnswer, JsonRpcRequest, JsonRpcResponse};
use chain_server::server::ChainProof;
use provider::BlockNumber;
use reqwest::Client;
use serde::{Deserialize, Serialize};
use tracing::info;

use crate::host::error::HostError;

#[derive(Debug, Serialize)]
struct ChainProofParams {
    chain_id: ChainId,
    block_numbers: HashSet<BlockNumber>,
}

pub struct ChainProofClient {
    base_url: String,
    http_client: Client,
}

impl ChainProofClient {
    pub fn new(base_url: String) -> Self {
        Self {
            base_url,
            http_client: Client::new(),
        }
    }

    pub async fn get_chain_proofs(
        &self,
        blocks_by_chain: HashMap<ChainId, HashSet<u64>>,
    ) -> Result<HashMap<ChainId, ChainProof>, HostError> {
        let mut chain_proofs = HashMap::new();

        for (chain_id, block_numbers) in blocks_by_chain {
            let proof = self.fetch_chain_proof(chain_id, &block_numbers).await?;
            chain_proofs.insert(chain_id, proof);
        }

        Ok(chain_proofs)
    }

    async fn fetch_chain_proof(
        &self,
        chain_id: ChainId,
        block_numbers: &HashSet<BlockNumber>,
    ) -> Result<ChainProof, HostError> {
        info!(
            "fetching chain proof for chain_id: {}, number of block_numbers: {}",
            chain_id,
            block_numbers.len()
        );
        let params = ChainProofParams {
            chain_id,
            block_numbers: block_numbers.clone(),
        };
        let params_value =
            serde_json::to_value(&params).map_err(|e| HostError::JsonParseError(e.to_string()))?;

        let json_rpc_request = JsonRpcRequest {
            method: "v_chain".to_string(),
            params: params_value,
            id: Id::Num(1), // TODO: use a real id
        };

        let response = self
            .http_client
            .post(&self.base_url)
            .json(&json_rpc_request)
            .send()
            .await
            .map_err(|e| HostError::HttpRequestFailed(e.to_string()))?;

        let response_body: JsonRpcResponse = response
            .json()
            .await
            .map_err(|e| HostError::JsonParseError(e.to_string()))?;

        match response_body.result {
            JsonRpcAnswer::Result(result_value) => {
                let chain_proof = serde_json::from_value(result_value)
                    .map_err(|e| HostError::JsonParseError(e.to_string()))?;
                Ok(chain_proof)
            }
            JsonRpcAnswer::Error(error) => Err(HostError::JsonRpcError(error.to_string())),
        }
    }
}

use std::collections::HashMap;

use alloy_primitives::{BlockHash, ChainId};
use chain_server::server::ChainProof as ChainProofRpc;
use chain_types::ChainProof;
use mpt::MerkleTrie;
use provider::BlockNumber;
use serde_json::json;
use server_utils::{RpcClient, RpcError};
use thiserror::Error;
use tracing::info;

pub struct ChainProofClient {
    rpc_client: RpcClient,
}

fn from_chain_proof_rpc(chain_proof_rpc: ChainProofRpc) -> ChainProof {
    ChainProof {
        proof: chain_proof_rpc.proof,
        mpt: MerkleTrie::from_rlp_nodes(chain_proof_rpc.nodes).unwrap(),
    }
}

#[derive(Debug, Error)]
pub enum ChainProofClientError {
    #[error("HTTP request failed: {0}")]
    HttpRequestFailed(String),
    #[error("JSON-RPC error: {0}")]
    JsonRpcError(String),
    #[error("JSON parse error: {0}")]
    JsonParseError(String),
}

impl From<RpcError> for ChainProofClientError {
    fn from(err: RpcError) -> Self {
        match err {
            RpcError::Http(err) => Self::HttpRequestFailed(err.to_string()),
            RpcError::JsonRpc(err) => Self::JsonRpcError(err.to_string()),
            RpcError::MissingResult => {
                Self::JsonParseError("Missing 'result' field in response".to_string())
            }
            RpcError::InvalidResponse(value) => Self::JsonParseError(value.to_string()),
        }
    }
}

impl ChainProofClient {
    pub fn new(base_url: impl AsRef<str>) -> Self {
        let rpc_client = RpcClient::new(base_url.as_ref(), "v_chain");
        Self { rpc_client }
    }

    pub async fn get_chain_proofs(
        &self,
        blocks_by_chain: HashMap<ChainId, HashMap<BlockNumber, BlockHash>>,
    ) -> Result<HashMap<ChainId, ChainProof>, ChainProofClientError> {
        let mut chain_id_to_chain_proof = HashMap::new();

        for (chain_id, blocks) in blocks_by_chain {
            let block_numbers: Vec<BlockNumber> = blocks.into_keys().collect();
            let proof = self.fetch_chain_proof(chain_id, block_numbers).await?;
            chain_id_to_chain_proof.insert(chain_id, proof);
        }

        Ok(chain_id_to_chain_proof)
    }

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

        let chain_proof_rpc: ChainProofRpc = serde_json::from_value(result_value)
            .map_err(|e| ChainProofClientError::JsonParseError(e.to_string()))?;

        let chain_proof = from_chain_proof_rpc(chain_proof_rpc);

        Ok(chain_proof)
    }
}

use std::collections::HashMap;

use alloy_primitives::{BlockHash, ChainId};
use chain_server::server::ChainProof;
use provider::BlockNumber;
use serde_json::json;
use server_utils::{RpcClient, RpcError};
use tracing::info;

use crate::host::error::HostError;

pub struct ChainProofClient {
    rpc_client: RpcClient,
}

impl ChainProofClient {
    pub fn new(base_url: impl AsRef<str>) -> Self {
        let rpc_client = RpcClient::new(base_url.as_ref(), "v_chain");
        Self { rpc_client }
    }

    pub async fn get_chain_proofs(
        &self,
        blocks_by_chain: HashMap<ChainId, HashMap<BlockNumber, BlockHash>>,
    ) -> Result<HashMap<ChainId, ChainProof>, HostError> {
        let mut chain_proofs = HashMap::new();

        for (chain_id, blocks) in blocks_by_chain {
            let block_numbers: Vec<BlockNumber> = blocks.into_keys().collect();
            let proof = self.fetch_chain_proof(chain_id, block_numbers).await?;
            chain_proofs.insert(chain_id, proof);
        }

        Ok(chain_proofs)
    }

    async fn fetch_chain_proof(
        &self,
        chain_id: ChainId,
        block_numbers: Vec<BlockNumber>,
    ) -> Result<ChainProof, HostError> {
        info!(
            "Fetching chain proof for chain_id: {}, block_numbers.len(): {}",
            chain_id,
            block_numbers.len()
        );

        let params = json!({
            "chain_id": chain_id,
            "block_numbers": block_numbers.clone(),
        });

        let result_value = self.rpc_client.call(&params).await.map_err(map_error)?;

        let chain_proof = serde_json::from_value(result_value)
            .map_err(|e| HostError::JsonParseError(e.to_string()))?;

        Ok(chain_proof)
    }
}

fn map_error(rpc_error: RpcError) -> HostError {
    match rpc_error {
        RpcError::Http(err) => HostError::HttpRequestFailed(err.to_string()),
        RpcError::JsonRpc(err) => HostError::JsonRpcError(err.to_string()),
        RpcError::MissingResult => {
            HostError::JsonParseError("Missing 'result' field in response".to_string())
        }
        RpcError::InvalidResponse(value) => HostError::JsonParseError(value.to_string()),
    }
}

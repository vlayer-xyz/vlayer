use std::collections::{HashMap, HashSet};

use alloy_primitives::ChainId;
use chain_server::server::ChainProof;
use provider::BlockNumber;
use serde::Serialize;
use server_utils::{RpcClient, RpcError};
use tracing::info;

use crate::host::error::HostError;

#[derive(Debug, Serialize)]
struct ChainProofParams {
    chain_id: ChainId,
    block_numbers: HashSet<BlockNumber>,
}

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
            "Fetching chain proof for chain_id: {}, block_numbers.len(): {}",
            chain_id,
            block_numbers.len()
        );

        let params = ChainProofParams {
            chain_id,
            block_numbers: block_numbers.clone(),
        };

        let result_value = self.rpc_client.call(&params).await.map_err(|e| match e {
            RpcError::Http(err) => HostError::HttpRequestFailed(err.to_string()),
            RpcError::JsonRpc(error) => HostError::JsonRpcError(error.to_string()),
            RpcError::MissingResult => {
                HostError::JsonParseError("Missing 'result' field in response".to_string())
            }
        })?;

        let chain_proof: ChainProof = serde_json::from_value(result_value)
            .map_err(|e| HostError::JsonParseError(e.to_string()))?;

        Ok(chain_proof)
    }
}

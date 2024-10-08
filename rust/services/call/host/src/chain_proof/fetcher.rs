use std::collections::HashSet;

use alloy_primitives::ChainId;
use chain_server::server::ChainProof;
use provider::BlockNumber;
use reqwest::Client;
use serde::{Deserialize, Serialize};
use tracing::info;

use crate::host::error::HostError;

#[derive(Debug, Serialize)]
struct ChainProofRequest {
    chain_id: ChainId,
    block_numbers: HashSet<BlockNumber>,
}

#[derive(Debug, Deserialize)]
struct ChainProofResponse {
    proof: ChainProof,
}

#[async_trait::async_trait]
pub trait ChainProofFetcherTrait: Send + Sync {
    async fn fetch_chain_proof(
        &self,
        chain_id: ChainId,
        block_numbers: &HashSet<BlockNumber>,
    ) -> Result<ChainProof, HostError>;
}

pub struct MockChainProofFetcher {}

#[async_trait::async_trait]
impl ChainProofFetcherTrait for MockChainProofFetcher {
    async fn fetch_chain_proof(
        &self,
        _chain_id: ChainId,
        _block_numbers: &HashSet<BlockNumber>,
    ) -> Result<ChainProof, HostError> {
        Ok(ChainProof::default())
    }
}

pub(crate) struct ChainProofFetcher {
    base_url: String,
    http_client: Client,
}

impl ChainProofFetcher {
    pub(crate) fn new(base_url: String, http_client: Client) -> Self {
        Self {
            base_url,
            http_client,
        }
    }
}

#[async_trait::async_trait]
impl ChainProofFetcherTrait for ChainProofFetcher {
    async fn fetch_chain_proof(
        &self,
        chain_id: ChainId,
        block_numbers: &HashSet<BlockNumber>,
    ) -> Result<ChainProof, HostError> {
        info!("Fetching chain proof for chain_id: {}", chain_id);
        let request_body = ChainProofRequest {
            chain_id,
            block_numbers: block_numbers.clone(),
        };

        let response = self
            .http_client
            .post(&self.base_url)
            .json(&request_body)
            .send()
            .await
            .map_err(|e| HostError::HttpRequestFailed(e.to_string()))?;

        if !response.status().is_success() {
            return Err(HostError::ServerErrorStatus(response.status()));
        }

        let response_body: ChainProofResponse = response
            .json()
            .await
            .map_err(|e| HostError::JsonParseError(e.to_string()))?;

        Ok(response_body.proof)
    }
}

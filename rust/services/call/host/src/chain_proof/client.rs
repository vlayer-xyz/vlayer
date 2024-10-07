use std::collections::{HashMap, HashSet};

use alloy_primitives::ChainId;
use chain_server::server::ChainProof;
use futures::future::join_all;
use reqwest::Client;

use super::fetcher::{ChainProofFetcher, ChainProofFetcherTrait};
use crate::host::error::HostError;

pub struct ChainProofClient {
    fetcher: Box<dyn ChainProofFetcherTrait + Send + Sync>,
}

impl ChainProofClient {
    pub fn new(chain_proof_url: String) -> Self {
        let http_client = Client::new();
        let fetcher = ChainProofFetcher::new(chain_proof_url, http_client);
        Self {
            fetcher: Box::new(fetcher),
        }
    }

    pub fn with_fetcher(fetcher: impl ChainProofFetcherTrait + 'static) -> Self {
        Self {
            fetcher: Box::new(fetcher),
        }
    }

    pub async fn get_chain_proofs(
        &self,
        blocks_by_chain: HashMap<ChainId, HashSet<u64>>,
    ) -> Result<HashMap<ChainId, ChainProof>, HostError> {
        let futures = blocks_by_chain
            .into_iter()
            .map(|(chain_id, block_numbers)| async move {
                let proof = self
                    .fetcher
                    .fetch_chain_proof(chain_id, &block_numbers)
                    .await?;
                Ok((chain_id, proof)) as Result<(ChainId, ChainProof), HostError>
            });

        let results = join_all(futures).await;

        let chain_proofs: HashMap<ChainId, ChainProof> = results
            .into_iter()
            .collect::<Result<HashMap<_, _>, _>>()?;

        Ok(chain_proofs)
    }
}

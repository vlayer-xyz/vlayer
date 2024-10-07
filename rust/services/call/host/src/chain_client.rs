use std::collections::{HashMap, HashSet};

use alloy_primitives::ChainId;
use chain_server::server::ChainProof;
use provider::BlockNumber;

use crate::host::error::HostError;

pub struct ChainProofClient;

impl ChainProofClient {
    pub fn get_chain_proofs(
        &self,
        blocks_by_chain: HashMap<ChainId, HashSet<u64>>,
    ) -> Result<HashMap<ChainId, ChainProof>, HostError> {
        let chain_proofs = blocks_by_chain
            .into_iter()
            .map(|(chain_id, block_numbers)| {
                self.fetch_chain_proofs(chain_id, &block_numbers)
                    .map(|proof| (chain_id, proof))
            })
            .collect::<Result<HashMap<_, _>, _>>()?;

        Ok(chain_proofs)
    }

    fn fetch_chain_proofs(
        &self,
        _chain_id: ChainId,
        _block_numbers: &HashSet<BlockNumber>,
    ) -> Result<ChainProof, HostError> {
        let _ = self;
        //todo: fetch real ChainProof data
        Ok(ChainProof::default())
    }
}

use std::collections::{HashMap, HashSet};

use alloy_primitives::ChainId;
use chain_server::server::ChainProof;
use provider::BlockNumber;

use crate::host::error::HostError;

pub struct ChainServer;

impl ChainServer {
    pub fn get_chain_proofs(
        &self,
        blocks_by_chain: HashMap<ChainId, HashSet<u64>>,
    ) -> Result<HashMap<ChainId, ChainProof>, HostError> {
        let mut chain_proofs = HashMap::new();

        for (chain_id, block_numbers) in blocks_by_chain {
            let chain_proof = self.fetch_chain_proofs(chain_id, &block_numbers)?;
            chain_proofs.insert(chain_id, chain_proof);
        }

        Ok(chain_proofs)
    }

    fn fetch_chain_proofs(
        &self,
        _chain_id: ChainId,
        _block_numbers: &HashSet<BlockNumber>,
    ) -> Result<ChainProof, HostError> {
        //todo: fetch real ChainProof data
        Ok(ChainProof::default())
    }
}

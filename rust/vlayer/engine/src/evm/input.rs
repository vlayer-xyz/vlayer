use alloy_primitives::Bytes;
use mpt::MerkleTrie;
use serde::{Deserialize, Serialize};
use tracing::debug;

use super::env::ExecutionLocation;

/// The serializable input to derive and validate a [EvmEnv].
#[derive(Debug, Serialize, Deserialize)]
pub struct EvmInput<H> {
    pub header: H,
    pub state_trie: MerkleTrie,
    pub storage_tries: Vec<MerkleTrie>,
    pub contracts: Vec<Bytes>,
    pub ancestors: Vec<H>,
}

impl<H> EvmInput<H> {
    pub fn print_sizes(&self) {
        let total_storage_size: usize = self.storage_tries.iter().map(|t| t.size()).sum();

        debug!("state size: {}", self.state_trie.size());
        debug!("storage tries: {}", self.storage_tries.len());
        debug!("total storage size: {}", total_storage_size);
        debug!("contracts: {}", self.contracts.len());
        debug!("blocks: {}", self.ancestors.len());
    }
}

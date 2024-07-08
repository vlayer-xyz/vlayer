use alloy_primitives::Bytes;
use mpt::MerkleTrie;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use tracing::debug;

use super::{block_header::EvmBlockHeader, env::ExecutionLocation};

/// The serializable input to derive and validate a [EvmEnv].
#[derive(Debug, Serialize, Deserialize, Clone)]
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

impl<H: EvmBlockHeader + Clone> EvmInput<H> {
    pub fn validate(&self) {
        // verify that the state root matches the state trie
        let state_root = self.state_trie.hash_slow();
        assert_eq!(self.header.state_root(), &state_root, "State root mismatch");

        // seal the header to compute its block hash
        let header = self.header.clone().seal_slow();
        // validate that ancestor headers form a valid chain
        let mut previous_header = header.inner();
        for ancestor in &self.ancestors {
            let ancestor_hash = ancestor.hash_slow();
            assert_eq!(
                previous_header.parent_hash(),
                &ancestor_hash,
                "Invalid chain: block {} is not the parent of block {}",
                ancestor.number(),
                previous_header.number()
            );
            previous_header = ancestor;
        }
    }
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct MultiEvmInput<H>(HashMap<ExecutionLocation, EvmInput<H>>);

impl<H> MultiEvmInput<H> {
    pub fn get(&self, location: &ExecutionLocation) -> Option<&EvmInput<H>> {
        self.0.get(location)
    }
}

impl<H> FromIterator<(ExecutionLocation, EvmInput<H>)> for MultiEvmInput<H> {
    fn from_iter<T: IntoIterator<Item = (ExecutionLocation, EvmInput<H>)>>(iter: T) -> Self {
        MultiEvmInput(iter.into_iter().collect())
    }
}

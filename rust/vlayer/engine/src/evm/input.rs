use alloy_primitives::{Bytes, B256};
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
    pub fn block_hashes(&self) -> HashMap<u64, B256> {
        let mut block_hashes = HashMap::with_capacity(self.ancestors.len() + 1);
        for ancestor in &self.ancestors {
            let ancestor_hash = ancestor.hash_slow();
            block_hashes.insert(ancestor.number(), ancestor_hash);
        }

        let header = self.header.clone().seal_slow();
        block_hashes.insert(self.header.number(), header.seal());

        block_hashes
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

use alloy_primitives::{Bytes, B256};
use mpt::MerkleTrie;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use tracing::debug;

use super::{block_header::EvmBlockHeader, env::ExecutionLocation};

/// The serializable input to derive and validate a [EvmEnv].
#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct EvmInput<H> {
    pub header: H,
    pub state_trie: MerkleTrie,
    pub storage_tries: Vec<MerkleTrie>,
    pub contracts: Vec<Bytes>,
    pub ancestors: Vec<H>,
}

impl<H: EvmBlockHeader> EvmInput<H> {
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

    pub fn validate_state_root(&self) {
        let state_root = self.state_trie.hash_slow();
        assert_eq!(self.header.state_root(), &state_root, "State root mismatch");
    }

    pub fn validate_ancestors(&self) {
        let header = self.header.clone().seal_slow();
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
pub struct MultiEvmInput<H>(pub HashMap<ExecutionLocation, EvmInput<H>>);

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

#[cfg(test)]
mod test {
    use mpt::EMPTY_ROOT_HASH;

    use crate::ethereum::EthBlockHeader;

    use super::EvmInput;

    mod validate_state_root {
        use super::*;

        #[test]
        fn success() {
            let input: EvmInput<EthBlockHeader> = EvmInput {
                header: EthBlockHeader {
                    state_root: EMPTY_ROOT_HASH,
                    ..Default::default()
                },
                ..Default::default()
            };
            input.validate_state_root();
        }

        #[test]
        #[should_panic(expected = "State root mismatch")]
        fn mismatch() {
            let input: EvmInput<EthBlockHeader> = Default::default();
            input.validate_state_root();
        }
    }

    mod validate_ancestors {
        use alloy_primitives::Sealable;

        use super::*;

        #[test]
        fn success() {
            let ancestor: EthBlockHeader = Default::default();
            let input: EvmInput<EthBlockHeader> = EvmInput {
                header: EthBlockHeader {
                    number: 1,
                    parent_hash: ancestor.hash_slow(),
                    ..Default::default()
                },
                ancestors: vec![Default::default()],
                ..Default::default()
            };
            input.validate_ancestors();
        }

        #[test]
        #[should_panic(expected = "failed: Invalid chain: block 0 is not the parent of block 1")]
        fn mismatch() {
            let input: EvmInput<EthBlockHeader> = EvmInput {
                header: EthBlockHeader {
                    number: 1,
                    ..Default::default()
                },
                ancestors: vec![Default::default()],
                ..Default::default()
            };
            input.validate_ancestors();
        }
    }
}

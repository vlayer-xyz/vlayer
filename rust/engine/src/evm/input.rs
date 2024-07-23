use alloy_primitives::{Bytes, B256};
use mpt::MerkleTrie;
use serde::{Deserialize, Serialize};
use std::{collections::HashMap, iter::once};
use tracing::debug;

use crate::block_header::EvmBlockHeader;

use super::env::location::ExecutionLocation;

/// The serializable input to derive and validate a [EvmEnv].
#[derive(Debug, Serialize, Deserialize)]
pub struct EvmInput {
    pub header: Box<dyn EvmBlockHeader>,
    pub state_trie: MerkleTrie,
    pub storage_tries: Vec<MerkleTrie>,
    pub contracts: Vec<Bytes>,
    pub ancestors: Vec<Box<dyn EvmBlockHeader>>,
}

impl EvmInput {
    pub fn print_sizes(&self) {
        let total_storage_size: usize = self.storage_tries.iter().map(|t| t.size()).sum();

        debug!("state size: {}", self.state_trie.size());
        debug!("storage tries: {}", self.storage_tries.len());
        debug!("total storage size: {}", total_storage_size);
        debug!("contracts: {}", self.contracts.len());
        debug!("blocks: {}", self.ancestors.len());
    }
}

impl EvmInput {
    pub fn block_hashes(&self) -> HashMap<u64, B256> {
        self.ancestors
            .iter()
            .map(|ancestor| (ancestor.number(), ancestor.hash_slow()))
            .chain(once((self.header.number(), self.header.hash_slow())))
            .collect()
    }

    pub fn validate_state_root(&self) {
        let state_root = self.state_trie.hash_slow();
        assert_eq!(self.header.state_root(), &state_root, "State root mismatch");
    }

    pub fn validate_ancestors(&self) {
        let mut previous_header = &self.header;
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

pub type MultiEvmInput = HashMap<ExecutionLocation, EvmInput>;

#[cfg(test)]
mod test {
    use mpt::EMPTY_ROOT_HASH;

    use super::EvmInput;
    use crate::block_header::eth::EthBlockHeader;
    use crate::block_header::{EvmBlockHeader, Hashable};

    use mpt::MerkleTrie;

    impl Default for EvmInput {
        fn default() -> Self {
            Self {
                header: Box::new(EthBlockHeader::default()),
                ancestors: vec![],
                state_trie: MerkleTrie::default(),
                storage_tries: Vec::default(),
                contracts: Vec::default(),
            }
        }
    }

    impl Default for Box<dyn EvmBlockHeader> {
        fn default() -> Self {
            Box::new(EthBlockHeader::default())
        }
    }

    mod block_hashes {
        use super::*;

        #[test]
        fn success() {
            let ancestor: EthBlockHeader = EthBlockHeader::default();
            let input = EvmInput {
                ancestors: vec![Default::default()],
                header: Box::new(EthBlockHeader {
                    number: 1,
                    parent_hash: ancestor.hash_slow(),
                    ..Default::default()
                }),
                ..Default::default()
            };
            let block_hashes = input.block_hashes();

            assert_eq!(block_hashes.len(), 2);
            assert_eq!(block_hashes.get(&0).unwrap(), &ancestor.hash_slow());
            assert_eq!(block_hashes.get(&1).unwrap(), &input.header.hash_slow());
        }
    }

    mod validate_state_root {

        use super::*;

        #[test]
        fn success() {
            let input = EvmInput {
                ancestors: vec![Default::default()],
                header: Box::new(EthBlockHeader {
                    state_root: EMPTY_ROOT_HASH,
                    ..Default::default()
                }),
                ..Default::default()
            };
            input.validate_state_root();
        }

        #[test]
        #[should_panic(expected = "State root mismatch")]
        fn mismatch() {
            let input = EvmInput::default();
            input.validate_state_root();
        }
    }

    mod validate_ancestors {
        use super::*;

        #[test]
        fn success() {
            let ancestor: EthBlockHeader = Default::default();
            let input = EvmInput {
                ancestors: vec![Default::default()],
                header: Box::new(EthBlockHeader {
                    number: 1,
                    parent_hash: ancestor.hash_slow(),
                    ..Default::default()
                }),
                ..Default::default()
            };

            input.validate_ancestors();
        }

        #[test]
        #[should_panic(expected = "failed: Invalid chain: block 0 is not the parent of block 1")]
        fn mismatch() {
            let input = EvmInput {
                ancestors: vec![Default::default()],
                header: Box::new(EthBlockHeader {
                    number: 1,
                    ..Default::default()
                }),
                ..Default::default()
            };
            input.validate_ancestors();
        }
    }
}

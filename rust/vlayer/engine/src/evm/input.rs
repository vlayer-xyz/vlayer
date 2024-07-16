use alloy_primitives::{Bytes, B256};
use mpt::MerkleTrie;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use tracing::debug;

use super::{block_header::EvmBlockHeader, env::ExecutionLocation};

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

    pub fn block_hashes(&self) -> HashMap<u64, B256> {
        let mut block_hashes = HashMap::with_capacity(self.ancestors.len() + 1);
        for ancestor in &self.ancestors {
            let ancestor_hash = ancestor.hash_slow();
            block_hashes.insert(ancestor.number(), ancestor_hash);
        }

        block_hashes.insert(self.header.number(), self.header.hash_slow());

        block_hashes
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

#[derive(Debug, Serialize, Deserialize)]
pub struct MultiEvmInput(pub HashMap<ExecutionLocation, EvmInput>);

impl MultiEvmInput {
    pub fn get(&self, location: &ExecutionLocation) -> Option<&EvmInput> {
        self.0.get(location)
    }
}

impl FromIterator<(ExecutionLocation, EvmInput)> for MultiEvmInput {
    fn from_iter<T: IntoIterator<Item = (ExecutionLocation, EvmInput)>>(iter: T) -> Self {
        MultiEvmInput(iter.into_iter().collect())
    }
}

#[cfg(test)]
mod test {
    use mpt::EMPTY_ROOT_HASH;

    use crate::ethereum::EthBlockHeader;
    use crate::evm::block_header::Hashable;
    use mpt::MerkleTrie;

    use super::EvmInput;

    mod block_hashes {

        use super::*;

        #[test]
        fn success() {
            let ancestor: EthBlockHeader = Default::default();
            let input: EvmInput = EvmInput {
                header: Box::new(EthBlockHeader {
                    number: 1,
                    parent_hash: ancestor.hash_slow(),
                    ..Default::default()
                }),
                ancestors: vec![Box::new(EthBlockHeader {
                    ..Default::default()
                })],
                state_trie: MerkleTrie::default(),
                storage_tries: vec![MerkleTrie::default()],
                contracts: vec![],
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
            let input: EvmInput = EvmInput {
                header: Box::new(EthBlockHeader {
                    state_root: EMPTY_ROOT_HASH,
                    ..Default::default()
                }),
                ancestors: vec![Box::new(EthBlockHeader {
                    ..Default::default()
                })],
                state_trie: MerkleTrie::default(),
                storage_tries: vec![MerkleTrie::default()],
                contracts: vec![],
            };
            input.validate_state_root();
        }

        // TODO: Uncomment this test and fix the initialization of EvmInput with default values.
        // #[test]
        // #[should_panic(expected = "State root mismatch")]
        // fn mismatch() {
        //     let input: EvmInput = Default::default();
        //     input.validate_state_root();
        // }
    }

    mod validate_ancestors {
        use super::*;

        #[test]
        fn success() {
            let ancestor: EthBlockHeader = Default::default();
            let input: EvmInput = EvmInput {
                header: Box::new(EthBlockHeader {
                    number: 1,
                    parent_hash: ancestor.hash_slow(),
                    ..Default::default()
                }),
                ancestors: vec![Box::new(EthBlockHeader {
                    ..Default::default()
                })],
                state_trie: MerkleTrie::default(),
                storage_tries: vec![MerkleTrie::default()],
                contracts: vec![],
            };
            input.validate_ancestors();
        }

        #[test]
        #[should_panic(expected = "failed: Invalid chain: block 0 is not the parent of block 1")]
        fn mismatch() {
            let input: EvmInput = EvmInput {
                header: Box::new(EthBlockHeader {
                    number: 1,

                    ..Default::default()
                }),
                ancestors: vec![Box::new(EthBlockHeader {
                    ..Default::default()
                })],
                state_trie: MerkleTrie::default(),
                storage_tries: vec![MerkleTrie::default()],
                contracts: vec![],
            };
            input.validate_ancestors();
        }
    }
}

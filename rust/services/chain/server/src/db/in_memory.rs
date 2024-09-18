use std::collections::HashMap;

use crate::db::block_database::BlockDbError;

use super::block_database::BlockDatabase;
use alloy_primitives::{fixed_bytes, FixedBytes};
use alloy_primitives::{keccak256, Bytes};
use hex::encode;
use mpt::MerkleTrie;

pub struct InMemoryBlockDb {
    hash_to_node: HashMap<String, Vec<u8>>,
    block_range_to_root: HashMap<(u32, u32), String>,
    block_range_to_proof: HashMap<(u32, u32), Bytes>,
    current_range: (u32, u32),
}

impl InMemoryBlockDb {
    #[allow(unused)]
    pub fn initialize() -> Self {
        let parent_hash: FixedBytes<32> =
            fixed_bytes!("88e96d4537bea4d9c05d12549907b32561d3bf31f45aae734cdc119f13406cb6"); // Block 1
        let child_hash: FixedBytes<32> =
            fixed_bytes!("b495a1d7e6663152ae92708da4843337b958146015a2802f4193a410044698c9"); // Block 2

        let trie = MerkleTrie::from_iter([
            (vec![1], parent_hash.clone()),
            (vec![2], child_hash.clone()),
        ]);

        let nodes: Vec<Bytes> = trie.to_rlp_nodes().map(Bytes::from).collect::<Vec<Bytes>>();

        let mut hash_to_node = HashMap::new();

        for node in &nodes {
            let hash = encode(keccak256(&node));
            hash_to_node.insert(hash, node.to_vec());
        }

        let block_range = (1, 2);
        let root_hash = encode(trie.hash_slow().to_vec());
        let proof = Bytes::default();

        let block_range_to_root = HashMap::from([(block_range, root_hash)]);
        let block_range_to_proof = HashMap::from([(block_range, proof)]);

        InMemoryBlockDb {
            hash_to_node,
            block_range_to_root,
            block_range_to_proof,
            current_range: block_range,
        }
    }
}

impl BlockDatabase for InMemoryBlockDb {
    fn get_proof(&self, block_numbers: impl AsRef<[u8]>) -> Result<&Vec<u8>, BlockDbError> {
        let block_numbers = block_numbers.as_ref();
        if block_numbers.is_empty() {
            return Err(BlockDbError::EmptyBlockNumbers);
        }
        let (start, end) = self.current_range;
        for &block_number in block_numbers {
            let block_number = block_number as u32;
            if block_number < start || block_number > end {
                return Err(BlockDbError::OutsideOfRange(
                    block_number,
                    self.current_range,
                ));
            }
        }
        todo!()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use lazy_static::lazy_static;

    lazy_static! {
        static ref DB: InMemoryBlockDb = InMemoryBlockDb::initialize();
    }

    #[test]
    fn empty_block_numbers() {
        let result = DB.get_proof([]);
        assert_eq!(result.unwrap_err(), BlockDbError::EmptyBlockNumbers);
    }

    #[test]
    fn single_block_outside_of_current_range() {
        let result = DB.get_proof([3]);
        assert_eq!(result.unwrap_err(), BlockDbError::OutsideOfRange(3, (1, 2)));
    }

    #[test]
    fn multiple_blocks_outside_of_current_range() {
        let result = DB.get_proof([3, 4]);
        assert_eq!(result.unwrap_err(), BlockDbError::OutsideOfRange(3, (1, 2)));
    }
}

use super::*;

use alloy_primitives::{keccak256, U256};
use alloy_trie::HashBuilder;
use nybbles::Nibbles;
use std::collections::BTreeMap;

mod e2e;
mod from_rlp_nodes;
mod get;
mod hash_slow;

#[test]
pub fn hash_sparse_mpt() {
    const NUM_LEAVES: usize = 1024;

    // populate leaves with hashed keys and RLP-encoded values
    let leaves: BTreeMap<_, _> = (0..NUM_LEAVES)
        .map(|i| {
            let key = U256::from(i);
            (
                Nibbles::unpack(keccak256(key.to_be_bytes::<32>())),
                alloy_rlp::encode(key),
            )
        })
        .collect();

    // generate proofs only for every second leaf
    let proof_keys = leaves.keys().step_by(2).cloned().collect();
    let mut hash_builder = HashBuilder::default().with_proof_retainer(proof_keys);
    for (key, value) in leaves {
        hash_builder.add_leaf(key, &value);
    }
    let root = hash_builder.root();
    let proofs = hash_builder.take_proofs();

    // reconstruct the trie from the RLP encoded proofs and verify the root hash
    let mpt = MerkleTrie::from_rlp_nodes(proofs.into_values())
        .expect("Failed to reconstruct Merkle Trie from proofs");
    assert_eq!(mpt.hash_slow(), root);
}

use std::collections::BTreeMap;

use alloy_primitives::U256;
use alloy_trie::HashBuilder;
use common::Hashable;
use mpt::{hash, reorder_with_root_as_first_using_keccak, KeccakMerkleTrie as MerkleTrie};
use nybbles::Nibbles;

type D = sha3::Keccak256;

#[test]
fn root_match() -> anyhow::Result<()> {
    const NUM_LEAVES: usize = 1024;

    // populate leaves with hashed keys and RLP-encoded values
    let leaves: BTreeMap<_, _> = (0..NUM_LEAVES)
        .map(|i| {
            let key = U256::from(i);
            (Nibbles::unpack(hash::<D>(key.to_be_bytes::<32>())), alloy_rlp::encode(key))
        })
        .collect();

    // generate proofs only for every second leaf
    let proof_keys = leaves.keys().step_by(2).cloned().collect();
    let mut hash_builder = HashBuilder::default().with_proof_retainer(proof_keys);
    for (key, value) in leaves {
        hash_builder.add_leaf(key, &value);
    }
    let root = hash_builder.root();
    let proofs = hash_builder.take_proof_nodes().into_inner();
    let nodes = reorder_with_root_as_first_using_keccak(proofs.values(), root);

    // reconstruct the trie from the RLP encoded proofs and verify the root hash
    let mpt = MerkleTrie::from_rlp_nodes(nodes)?;

    assert_eq!(mpt.hash_slow(), root);
    Ok(())
}

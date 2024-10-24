use alloy_primitives::{keccak256, B256};
use itertools::Itertools;

// Alloy does not return node as first element in the proof, so we need to reorder it
pub fn reorder_with_root_as_first<T: AsRef<[u8]>>(
    nodes: impl Iterator<Item = T>,
    root_hash: B256,
) -> Vec<T> {
    let mut nodes: Vec<T> = nodes.collect();
    let root_position = nodes
        .iter()
        .find_position(|item| keccak256(item) == root_hash)
        .expect("No root node found")
        .0;
    nodes.swap(root_position, 0);
    nodes
}

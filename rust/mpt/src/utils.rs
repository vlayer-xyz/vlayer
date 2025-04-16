use alloy_primitives::B256;
use itertools::Itertools;

use crate::{Digest, hash};

// Alloy does not return node as first element in the proof, so we need to reorder it
#[allow(clippy::expect_used)]
pub fn reorder_root_first<T, D>(nodes: impl Iterator<Item = T>, root_hash: B256) -> Vec<T>
where
    T: AsRef<[u8]>,
    D: Digest,
{
    let mut nodes: Vec<T> = nodes.collect();
    let root_position = nodes
        .iter()
        .find_position(|item| hash::<D>(item) == root_hash)
        .expect("No root node found")
        .0;
    nodes.swap(root_position, 0);
    nodes
}

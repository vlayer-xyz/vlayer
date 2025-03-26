use mpt::KeccakMerkleTrie as MerkleTrie;
use rand::{SeedableRng, rngs::StdRng, seq::SliceRandom};

use crate::utils::generate_entries_with_unique_keys;

mod utils;

#[test]
fn insert_is_commutative() {
    let mut rng = StdRng::seed_from_u64(0);

    let mut entries = generate_entries_with_unique_keys(&mut rng);
    let trie = MerkleTrie::from_iter(entries.clone());

    entries.shuffle(&mut rng);
    let trie_with_elements_shuffled = MerkleTrie::from_iter(entries);

    assert_eq!(trie, trie_with_elements_shuffled);
}

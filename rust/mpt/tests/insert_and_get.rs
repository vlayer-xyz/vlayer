use mpt::KeccakMerkleTrie as MerkleTrie;
use rand::{SeedableRng, rngs::StdRng};
use utils::generate_entries_with_unique_keys;

mod utils;

#[test]
fn insert_and_get() {
    let mut rng = StdRng::seed_from_u64(0);

    let entries = generate_entries_with_unique_keys(&mut rng);
    let trie = MerkleTrie::from_iter(entries.clone());

    for (key, expected_value) in entries {
        let retrieved_value = trie.get(key).unwrap();
        assert_eq!(retrieved_value, expected_value);
    }
}

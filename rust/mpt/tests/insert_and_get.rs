use mpt::MerkleTrie;
use rand::rngs::StdRng;
use rand::SeedableRng;
use utils::create_elements;

mod utils;

#[test]
fn insert_and_get() {
    let mut rng = StdRng::seed_from_u64(0);

    let elements = create_elements(&mut rng);
    let trie = MerkleTrie::from_iter(elements.clone());

    for (key, expected_value) in elements {
        let retrieved_value = trie.get(key).unwrap();
        assert_eq!(retrieved_value, expected_value);
    }
}

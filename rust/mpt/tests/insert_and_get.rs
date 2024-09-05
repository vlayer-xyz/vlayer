use rand::rngs::StdRng;
use rand::SeedableRng;
use utils::{create_elements, create_trie_with_elements_inserted};

mod utils;

#[test]
fn retrieve_inserted_elements() {
    let seed: [u8; 32] = [0; 32];
    let mut rng = StdRng::from_seed(seed);

    let elements = create_elements(&mut rng);
    let trie = create_trie_with_elements_inserted(&elements);

    for (key, expected_value) in elements {
        let retrieved_value = trie.get(key).unwrap();
        assert_eq!(retrieved_value, expected_value);
    }
}

use rand::{rngs::StdRng, SeedableRng};

use crate::utils::{create_elements, create_trie_with_elements_inserted, shuffle_elements};

mod utils;

#[test]
fn structure_independent_of_insert_order() {
    let seed: [u8; 32] = [0; 32];
    let mut rng = StdRng::from_seed(seed);

    let elements = create_elements(&mut rng);
    let trie = create_trie_with_elements_inserted(&elements);

    let shuffled_elements = shuffle_elements(elements, &mut rng);
    let trie_with_elements_shuffled = create_trie_with_elements_inserted(&shuffled_elements);

    assert_eq!(trie, trie_with_elements_shuffled);
}

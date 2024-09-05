use mpt::MerkleTrie;
use rand::rngs::StdRng;
use rand::seq::SliceRandom;
use rand::SeedableRng;

use crate::utils::create_elements;

mod utils;

#[test]
fn structure_independent_of_insert_order() {
    let mut rng = StdRng::seed_from_u64(0);

    let mut elements = create_elements(&mut rng);
    let trie = MerkleTrie::from_iter(elements.clone());

    elements.shuffle(&mut rng);
    let trie_with_elements_shuffled = MerkleTrie::from_iter(elements);

    assert_eq!(trie, trie_with_elements_shuffled);
}

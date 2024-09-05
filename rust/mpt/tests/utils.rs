use std::collections::HashSet;

use mpt::MerkleTrie;
use rand::rngs::StdRng;
use rand::seq::SliceRandom;
use rand::Rng;

const MAX_KEY_LENGTH: u8 = 8;
const MAX_VALUE_PER_BYTE: u8 = 5;
const NUMBER_OF_ELEMENTS: usize = 2048;

#[allow(unused)]
pub(crate) fn create_elements(rng: &mut StdRng) -> Vec<(Vec<u8>, Vec<u8>)> {
    let mut unique_keys = HashSet::new();
    let mut elements = Vec::new();

    while elements.len() < NUMBER_OF_ELEMENTS {
        let key: Vec<u8> = (0..rng.gen_range(0..=MAX_KEY_LENGTH))
            .map(|_| rng.gen_range(0..MAX_VALUE_PER_BYTE))
            .collect();

        if unique_keys.insert(key.clone()) {
            let value: Vec<u8> = rng.gen::<[u8; 32]>().to_vec();
            elements.push((key, value));
        }
    }
    elements
}

#[allow(unused)]
pub(crate) fn create_trie_with_elements_inserted(elements: &[(Vec<u8>, Vec<u8>)]) -> MerkleTrie {
    let mut trie = MerkleTrie::new();
    for (key, value) in elements {
        trie.insert(key, value).expect("Insert failed");
    }
    trie
}

#[allow(unused)]
pub(crate) fn shuffle_elements(
    mut elements: Vec<(Vec<u8>, Vec<u8>)>,
    rng: &mut StdRng,
) -> Vec<(Vec<u8>, Vec<u8>)> {
    elements.shuffle(rng);
    elements
}

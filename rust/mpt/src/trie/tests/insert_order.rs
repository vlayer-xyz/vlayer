#[cfg(test)]
mod tests {
    use crate::MerkleTrie;
    use std::collections::HashSet;

    use rand::rngs::StdRng;
    use rand::seq::SliceRandom;
    use rand::Rng;
    use rand::SeedableRng;

    const MAX_KEY_LENGTH: u8 = 6;
    const MAX_VALUE_PER_BYTE: u8 = 8;

    fn create_elements(
        max_key_length: u8,
        max_value_per_byte: u8,
        rng: &mut StdRng,
    ) -> Vec<(Vec<u8>, Vec<u8>)> {
        let mut unique_keys = HashSet::new();
        let mut elements = Vec::new();

        while elements.len() < 1024 {
            let key: Vec<u8> = (0..rng.gen_range(0..=max_key_length))
                .map(|_| rng.gen_range(0..max_value_per_byte))
                .collect();
    
            if unique_keys.insert(key.clone()) {
                let value: Vec<u8> = rng.gen::<[u8; 32]>().to_vec();
                elements.push((key, value));
            }
        }
        elements
    }

    fn create_trie_with_elements_inserted(elements: &[(Vec<u8>, Vec<u8>)]) -> MerkleTrie {
        let mut trie = MerkleTrie::new();
        for (key, value) in elements {
            trie.insert(&key, &value).expect("Insert failed");
        }
        trie
    }

    fn shuffle_elements(
        elements: &[(Vec<u8>, Vec<u8>)],
        rng: &mut StdRng,
    ) -> Vec<(Vec<u8>, Vec<u8>)> {
        let mut shuffled_elements = elements.to_vec();
        shuffled_elements.shuffle(rng);
        shuffled_elements
    }

    #[test]
    fn insert_order() {
        let seed: [u8; 32] = [0; 32];
        let mut rng = StdRng::from_seed(seed);

        let elements = create_elements(MAX_KEY_LENGTH, MAX_VALUE_PER_BYTE, &mut rng);
        let trie = create_trie_with_elements_inserted(&elements);

        let shuffled_elements = shuffle_elements(&elements, &mut rng);
        let trie_with_elements_shuffled = create_trie_with_elements_inserted(&shuffled_elements);

        assert_eq!(trie, trie_with_elements_shuffled);
    }

    #[test]
    fn retrieve_inserted_elements() {
        let seed: [u8; 32] = [0; 32];
        let mut rng = StdRng::from_seed(seed);

        let elements = create_elements(MAX_KEY_LENGTH, MAX_VALUE_PER_BYTE, &mut rng);
        let trie = create_trie_with_elements_inserted(&elements);

        for (key, expected_value) in elements {
            let retrieved_value = trie.get(key).unwrap();
            assert_eq!(retrieved_value, expected_value);
        }
    }
}

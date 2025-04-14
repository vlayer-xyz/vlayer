use itertools::Itertools;
use rand::{Rng, distributions::Uniform, rngs::StdRng};

const MAX_KEY_LENGTH: usize = 8;
const MAX_VALUE_PER_BYTE: u8 = 5;
const NUMBER_OF_ELEMENTS: usize = 1000;

type Entry = (Vec<u8>, [u8; 32]);

fn generate_key(rng: &mut StdRng) -> Vec<u8> {
    let key_byte_distr = Uniform::from(0..MAX_VALUE_PER_BYTE);
    let len = rng.gen_range(0..=MAX_KEY_LENGTH);
    rng.sample_iter(key_byte_distr).take(len).collect()
}

fn generate_entry(rng: &mut StdRng) -> Entry {
    let key = generate_key(rng);
    let value = rng.r#gen();
    (key, value)
}

#[allow(unused)]
pub(crate) fn generate_entries_with_unique_keys(rng: &mut StdRng) -> Vec<Entry> {
    (0..)
        .map(|_| generate_entry(rng))
        .unique_by(|(key, _)| key.clone())
        .take(NUMBER_OF_ELEMENTS)
        .collect()
}

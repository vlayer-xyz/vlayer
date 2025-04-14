use common::Hashable;
use mpt::KeccakMerkleTrie as MerkleTrie;

use crate::{Benchmark, benchmarks::merge, with_fixture};

mod empty {
    use super::*;

    fn hash(trie: MerkleTrie) {
        trie.hash_slow();
    }

    fn insert(mut trie: MerkleTrie) {
        trie.insert([0], [0; 32]).unwrap();
    }

    pub fn benchmarks() -> Vec<Benchmark> {
        vec![
            Benchmark::new("hash", with_fixture!(MerkleTrie::new(), hash), 160),
            Benchmark::new("insert", with_fixture!(MerkleTrie::new(), insert), 1_303),
        ]
    }
}

mod height_20 {
    use std::iter;

    use super::*;

    const HEIGHT: usize = 20;

    fn zeros(size: usize) -> Vec<u8> {
        iter::repeat(0).take(size).collect()
    }

    fn fixture() -> MerkleTrie {
        let mut trie = MerkleTrie::new();
        for i in 0..HEIGHT {
            let key = zeros(i);
            trie.insert(&key, [0; 32]).unwrap();
        }
        trie
    }

    fn insert_shallow(mut trie: MerkleTrie) {
        trie.insert([1], [0; 32]).unwrap();
    }

    fn insert_deep(mut trie: MerkleTrie) {
        trie.insert(zeros(HEIGHT), [0; 32]).unwrap();
    }

    fn hash(trie: MerkleTrie) {
        trie.hash_slow();
    }

    pub fn benchmarks() -> Vec<Benchmark> {
        vec![
            Benchmark::new("insert_shallow", with_fixture!(fixture(), insert_shallow), 23_808),
            Benchmark::new("insert_deep", with_fixture!(fixture(), insert_deep), 152_216),
            Benchmark::new("hash", with_fixture!(fixture(), hash), 148_794),
        ]
    }
}

pub fn benchmarks() -> Vec<Benchmark> {
    merge([("empty", empty::benchmarks()), ("height_20", height_20::benchmarks())])
}

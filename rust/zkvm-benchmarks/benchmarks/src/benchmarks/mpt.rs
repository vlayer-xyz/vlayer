use common::Hashable;
use mpt::MerkleTrie;

use crate::{benchmarks::merge, Benchmark};

mod empty {

    use super::*;

    fn trie() {
        MerkleTrie::new();
    }

    fn hash() {
        let trie = MerkleTrie::new();
        trie.hash_slow();
    }

    fn insert() {
        let mut trie = MerkleTrie::new();
        trie.insert([0], [0; 32]).unwrap();
    }

    pub fn benchmarks() -> Vec<Benchmark> {
        vec![
            Benchmark::new("trie", trie, 47),
            Benchmark::new("hash", hash, 122),
            Benchmark::new("insert", insert, 1_279),
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
            Benchmark::new(
                "insert_shallow",
                {
                    let trie = fixture();
                    move || insert_shallow(trie)
                },
                27_662,
            ),
            Benchmark::new(
                "insert_deep",
                {
                    let trie = fixture();
                    move || insert_deep(trie)
                },
                174_654,
            ),
            Benchmark::new(
                "hash",
                {
                    let trie = fixture();
                    move || hash(trie)
                },
                1_057_548,
            ),
        ]
    }
}

pub fn benchmarks() -> Vec<Benchmark> {
    merge([("empty", empty::benchmarks()), ("height_20", height_20::benchmarks())])
}

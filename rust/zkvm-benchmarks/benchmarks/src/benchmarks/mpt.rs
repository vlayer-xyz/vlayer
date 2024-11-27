use common::Hashable;
use mpt::MerkleTrie;

use crate::{benchmarks::merge, Benchmark, WorkloadResult};

mod empty {

    use super::*;

    fn trie() -> WorkloadResult {
        MerkleTrie::new();

        Ok(())
    }

    fn hash() -> WorkloadResult {
        let trie = MerkleTrie::new();
        trie.hash_slow();

        Ok(())
    }

    fn insert() -> WorkloadResult {
        let mut trie = MerkleTrie::new();
        trie.insert([0], [0; 32]).unwrap();

        Ok(())
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

    fn trie() -> WorkloadResult {
        fixture();

        Ok(())
    }

    fn insert_shallow() -> WorkloadResult {
        let mut trie = fixture();
        trie.insert([1], [0; 32]).unwrap();

        Ok(())
    }

    fn insert_deep() -> WorkloadResult {
        let mut trie = fixture();
        trie.insert(zeros(HEIGHT), [0; 32]).unwrap();

        Ok(())
    }

    fn hash() -> WorkloadResult {
        fixture().hash_slow();

        Ok(())
    }

    pub fn benchmarks() -> Vec<Benchmark> {
        vec![
            Benchmark::new("trie", trie, 1_264_090),
            Benchmark::new("insert_shallow", insert_shallow, 1_289_310),
            Benchmark::new("insert_deep", insert_deep, 1_435_893),
            Benchmark::new("hash", hash, 2_319_286),
        ]
    }
}

pub fn benchmarks() -> Vec<Benchmark> {
    merge([("empty", empty::benchmarks()), ("height_20", height_20::benchmarks())])
}

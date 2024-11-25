use common::Hashable;
use lazy_static::lazy_static;
use mpt::MerkleTrie;

use crate::{benchmarks::merge, Benchmark, Workload, WorkloadResult};

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

    lazy_static! {
        pub static ref BENCHMARKS: Vec<Benchmark> = vec![
            Benchmark::new("trie", trie as Workload, 1_200),
            Benchmark::new("hash", hash as Workload, 1_200),
            Benchmark::new("insert", insert as Workload, 1_200),
        ];
    }
}

mod height_4 {
    use std::iter;

    use super::*;

    const HEIGHT: usize = 4;

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

    fn hash() -> WorkloadResult {
        fixture().hash_slow();

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

    lazy_static! {
        pub static ref BENCHMARKS: Vec<Benchmark> = vec![
            Benchmark::new("trie", trie as Workload, 23_200),
            Benchmark::new("hash", hash as Workload, 1_900_000),
            Benchmark::new("insert_shallow", insert_shallow as Workload, 32_000),
            Benchmark::new("insert_deep", insert_deep as Workload, 40_000),
        ];
    }
}

lazy_static! {
    pub static ref BENCHMARKS: Vec<Benchmark> = {
        merge([("empty", empty::BENCHMARKS.clone()), ("height_4", height_4::BENCHMARKS.clone())])
    };
}

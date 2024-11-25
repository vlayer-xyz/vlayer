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

    lazy_static! {
        pub static ref BENCHMARKS: Vec<Benchmark> = vec![
            Benchmark::new("trie", trie as Workload, 1_300_000),
            Benchmark::new("insert_shallow", insert_shallow as Workload, 1_300_000),
            Benchmark::new("insert_deep", insert_deep as Workload, 1_500_000),
            Benchmark::new("hash", hash as Workload, 2_500_000),
        ];
    }
}

lazy_static! {
    pub static ref BENCHMARKS: Vec<Benchmark> = {
        merge([
            ("empty", empty::BENCHMARKS.clone()),
            ("height_20", height_20::BENCHMARKS.clone()),
        ])
    };
}

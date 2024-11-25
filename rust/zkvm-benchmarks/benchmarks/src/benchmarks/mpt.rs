use common::Hashable;
use mpt::MerkleTrie;

use crate::WorkloadResult;

pub(crate) mod empty {
    use super::*;

    pub(crate) fn trie() -> WorkloadResult {
        MerkleTrie::new();

        Ok(())
    }

    pub(crate) fn hash() -> WorkloadResult {
        let trie = MerkleTrie::new();
        trie.hash_slow();

        Ok(())
    }

    pub(crate) fn insert() -> WorkloadResult {
        let mut trie = MerkleTrie::new();
        trie.insert([0], [0; 32]).unwrap();

        Ok(())
    }
}

pub(crate) mod height_4 {
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

    pub(crate) fn trie() -> WorkloadResult {
        fixture();

        Ok(())
    }

    pub(crate) fn hash() -> WorkloadResult {
        fixture().hash_slow();

        Ok(())
    }

    pub(crate) fn insert_shallow() -> WorkloadResult {
        let mut trie = fixture();
        trie.insert([1], [0; 32]).unwrap();

        Ok(())
    }

    pub(crate) fn insert_deep() -> WorkloadResult {
        let mut trie = fixture();
        trie.insert(zeros(HEIGHT), [0; 32]).unwrap();

        Ok(())
    }
}

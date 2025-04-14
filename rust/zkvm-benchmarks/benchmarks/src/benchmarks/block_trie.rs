use ::block_trie::BlockTrie;
use block_header::{EvmBlockHeader, test_utils::mock_block_headers};

use crate::{Benchmark, with_fixture};

mod append_single {
    use super::*;

    pub fn fixture() -> (BlockTrie, Box<dyn EvmBlockHeader>) {
        let [block0, block1] = mock_block_headers(0..=1).try_into().unwrap();
        (BlockTrie::init(block0).unwrap(), block1)
    }

    pub fn run((mut trie, block1): (BlockTrie, Box<dyn EvmBlockHeader>)) {
        trie.append_single(&block1).unwrap();
    }
}

mod prepend_single {
    use super::*;

    pub fn fixture() -> (BlockTrie, Box<dyn EvmBlockHeader>) {
        let [_, block1] = mock_block_headers(0..=1).try_into().unwrap();
        (BlockTrie::init(&block1).unwrap(), block1)
    }

    pub fn run((mut trie, block1): (BlockTrie, Box<dyn EvmBlockHeader>)) {
        trie.prepend_single(&block1).unwrap();
    }
}

mod append_batch {
    use super::*;

    pub fn fixture(size: u64) -> (BlockTrie, Vec<Box<dyn EvmBlockHeader>>) {
        let mut blocks = mock_block_headers(0..=size);
        let trie = BlockTrie::init(blocks.remove(0)).unwrap();
        (trie, blocks)
    }

    pub fn run((mut trie, blocks): (BlockTrie, Vec<Box<dyn EvmBlockHeader>>)) {
        trie.append(blocks.into_iter()).unwrap()
    }
}

mod prepend_batch {
    use super::*;

    pub fn fixture(
        size: u64,
    ) -> (BlockTrie, Vec<Box<dyn EvmBlockHeader>>, Box<dyn EvmBlockHeader>) {
        let mut blocks = mock_block_headers(0..=size);
        #[allow(clippy::cast_possible_truncation)]
        let old_leftmost = blocks.remove(size as usize);
        let trie = BlockTrie::init(&old_leftmost).unwrap();
        (trie, blocks, old_leftmost)
    }

    pub fn run(
        (mut trie, blocks, old_leftmost): (
            BlockTrie,
            Vec<Box<dyn EvmBlockHeader>>,
            Box<dyn EvmBlockHeader>,
        ),
    ) {
        trie.prepend(blocks.into_iter(), old_leftmost).unwrap()
    }
}

mod hash_slow {
    use common::Hashable;

    use super::*;

    pub fn fixture(size: u64) -> BlockTrie {
        let mut blocks = mock_block_headers(0..=size);
        let mut block_trie = BlockTrie::init(blocks.remove(0)).unwrap();
        block_trie.append(blocks.into_iter()).unwrap();
        block_trie
    }

    pub fn run(trie: BlockTrie) {
        _ = trie.hash_slow();
    }
}

pub fn benchmarks() -> Vec<Benchmark> {
    vec![
        Benchmark::new(
            "append_single",
            with_fixture!(append_single::fixture(), append_single::run),
            16_461,
        ),
        Benchmark::new(
            "prepend_single",
            with_fixture!(prepend_single::fixture(), prepend_single::run),
            16_465,
        ),
        Benchmark::new(
            "append_10",
            with_fixture!(append_batch::fixture(10), append_batch::run),
            177_856,
        ),
        Benchmark::new(
            "prepend_10",
            with_fixture!(prepend_batch::fixture(10), prepend_batch::run),
            185_934,
        ),
        Benchmark::new(
            "prepend_20",
            with_fixture!(prepend_batch::fixture(20), prepend_batch::run),
            378_357,
        ),
        Benchmark::new("hash_100", with_fixture!(hash_slow::fixture(100), hash_slow::run), 235_021),
    ]
}

use ::block_trie::KeccakBlockTrie as BlockTrie;
use block_header::{test_utils::mock_block_headers, EvmBlockHeader};

use crate::{with_fixture, Benchmark};

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
        let mut blocks = mock_block_headers(0..=(size));
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

pub fn benchmarks() -> Vec<Benchmark> {
    vec![
        Benchmark::new(
            "append_single",
            with_fixture!(append_single::fixture(), append_single::run),
            113_026,
        ),
        Benchmark::new(
            "prepend_single",
            with_fixture!(prepend_single::fixture(), prepend_single::run),
            112_947,
        ),
        Benchmark::new(
            "append_10",
            with_fixture!(append_batch::fixture(10), append_batch::run),
            1_145_303,
        ),
        Benchmark::new(
            "prepend_10",
            with_fixture!(prepend_batch::fixture(10), prepend_batch::run),
            1_159_542,
        ),
        Benchmark::new(
            "prepend_20",
            with_fixture!(prepend_batch::fixture(20), prepend_batch::run),
            2_325_021,
        ),
    ]
}

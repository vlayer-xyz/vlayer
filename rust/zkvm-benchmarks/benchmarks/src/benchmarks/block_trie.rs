use ::block_trie::KeccakBlockTrie as BlockTrie;
use block_header::{EthBlockHeader as Block, EvmBlockHeader};

use crate::{with_fixture, Benchmark};

fn dummy_block(n: u64) -> Box<dyn EvmBlockHeader> {
    let block = Block {
        number: n,
        ..Default::default()
    };
    Box::new(block)
}

fn dummy_block_with_correct_parent_hash(n: u64) -> Box<dyn EvmBlockHeader> {
    if n == 0 {
        return dummy_block(0);
    }
    let block = Block {
        number: n,
        parent_hash: dummy_block_with_correct_parent_hash(n - 1).hash_slow(),
        ..Default::default()
    };
    Box::new(block)
}

mod append_single {
    use super::*;

    pub fn fixture() -> (BlockTrie, Box<dyn EvmBlockHeader>) {
        (
            BlockTrie::init(dummy_block(0)).unwrap(),
            dummy_block_with_correct_parent_hash(1),
        )
    }

    pub fn run((mut trie, block_one): (BlockTrie, Box<dyn EvmBlockHeader>)) {
        trie.append_single(&block_one).unwrap();
    }
}

mod prepend_single {
    use super::*;

    pub fn fixture() -> (BlockTrie, Box<dyn EvmBlockHeader>) {
        (
            BlockTrie::init(dummy_block_with_correct_parent_hash(1)).unwrap(),
            dummy_block_with_correct_parent_hash(1),
        )
    }

    pub fn run((mut trie, block_one): (BlockTrie, Box<dyn EvmBlockHeader>)) {
        trie.prepend_single(&block_one).unwrap();
    }
}

mod append_batch {
    use super::*;

    pub fn fixture(size: u64) -> (BlockTrie, Vec<Box<dyn EvmBlockHeader>>) {
        let trie = BlockTrie::init(dummy_block(0)).unwrap();
        let blocks = (1..=size)
            .map(dummy_block_with_correct_parent_hash)
            .collect();
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
        let old_leftmost = dummy_block_with_correct_parent_hash(size);
        let trie = BlockTrie::init(&old_leftmost).unwrap();
        let blocks = (0..size)
            .map(dummy_block_with_correct_parent_hash)
            .collect();
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

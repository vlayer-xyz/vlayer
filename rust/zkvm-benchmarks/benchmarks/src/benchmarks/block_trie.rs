use ::block_trie::BlockTrie;
use block_header::{EthBlockHeader as Block, EvmBlockHeader};

use crate::{with_fixture, Benchmark};

fn block(n: u64) -> Box<dyn EvmBlockHeader> {
    let block = Block {
        number: n,
        ..Default::default()
    };
    Box::new(block)
}

fn block_with_correct_parent_hash(n: u64) -> Box<dyn EvmBlockHeader> {
    let block = Block {
        number: n,
        parent_hash: block(n - 1).hash_slow(),
        ..Default::default()
    };
    Box::new(block)
}

mod append_single {
    use super::*;

    pub fn fixture() -> (BlockTrie, Box<dyn EvmBlockHeader>) {
        (BlockTrie::init(block(0)).unwrap(), block_with_correct_parent_hash(1))
    }

    pub fn run((mut trie, block_one): (BlockTrie, Box<dyn EvmBlockHeader>)) {
        trie.append_single(&block_one).unwrap();
    }
}

mod prepend_single {
    use super::*;

    pub fn fixture() -> (BlockTrie, Box<dyn EvmBlockHeader>) {
        (
            BlockTrie::init(block_with_correct_parent_hash(1)).unwrap(),
            block_with_correct_parent_hash(1),
        )
    }

    pub fn run((mut trie, block_one): (BlockTrie, Box<dyn EvmBlockHeader>)) {
        trie.prepend_single(&block_one).unwrap();
    }
}

pub fn benchmarks() -> Vec<Benchmark> {
    vec![
        Benchmark::new(
            "append_single",
            with_fixture!(append_single::fixture(), append_single::run),
            113_000,
        ),
        Benchmark::new(
            "prepend_single",
            with_fixture!(prepend_single::fixture(), prepend_single::run),
            113_000,
        ),
    ]
}

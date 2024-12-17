use std::ops::RangeInclusive;

use alloy_primitives::{BlockNumber, B256};
use block_header::{test_utils::mock_block_headers, EvmBlockHeader};
use block_trie::BlockTrie;
use common::Hashable;
use risc0_zkvm::{serde::to_vec, sha::Digest, FakeReceipt, InnerReceipt, ReceiptClaim};

mod chain_proof;
mod guest_input;

const CHAIN_GUEST_ID: Digest = Digest::new([0, 0, 0, 0, 0, 0, 0, 1]);

fn mock_block_trie(blocks: RangeInclusive<BlockNumber>) -> BlockTrie {
    let mut trie = BlockTrie::default();
    for header in mock_block_headers(blocks) {
        trie.insert_unchecked(header.number(), &header.hash_slow())
            .unwrap();
    }
    trie
}

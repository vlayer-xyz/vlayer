use std::ops::RangeInclusive;

use alloy_primitives::{BlockNumber, B256};
use block_header::{EthBlockHeader, EvmBlockHeader};
use block_trie::BlockTrie;
use common::Hashable;
use risc0_zkvm::{serde::to_vec, sha::Digest, FakeReceipt, InnerReceipt, ReceiptClaim};

use super::*;

mod chain_proof;
mod guest_input;

const CHAIN_GUEST_ID: Digest = Digest::new([0, 0, 0, 0, 0, 0, 0, 1]);

fn mock_block_header(number: BlockNumber, parent_hash: B256) -> Box<dyn EvmBlockHeader> {
    Box::new(EthBlockHeader {
        number,
        parent_hash,
        ..Default::default()
    })
}

fn mock_block_headers(blocks: RangeInclusive<BlockNumber>) -> Vec<Box<dyn EvmBlockHeader>> {
    let mut headers = vec![];
    let mut parent_hash = B256::default();
    for number in blocks {
        let header = mock_block_header(number, parent_hash);
        parent_hash = header.hash_slow();
        headers.push(header);
    }
    headers
}

fn mock_block_trie(blocks: RangeInclusive<BlockNumber>) -> BlockTrie {
    let mut trie = BlockTrie::default();
    for header in mock_block_headers(blocks) {
        trie.insert_unchecked(header.number(), &header.hash_slow())
            .unwrap();
    }
    trie
}

use std::ops::RangeInclusive;

use alloy_primitives::{BlockNumber, B256};
use block_header::test_utils::mock_block_headers;
use block_trie::BlockTrie;
use common::Hashable;
use risc0_zkvm::{serde::to_vec, sha::Digest, FakeReceipt, InnerReceipt, ReceiptClaim};

mod chain_proof;
mod time_travel;

const CHAIN_GUEST_ID: Digest = Digest::new([0, 0, 0, 0, 0, 0, 0, 1]);

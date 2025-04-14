use std::ops::RangeInclusive;

use alloy_primitives::{B256, BlockNumber};
use block_header::test_utils::mock_block_headers;
use block_trie::BlockTrie;

mod teleport;
mod time_travel;

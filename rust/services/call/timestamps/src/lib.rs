use std::{collections::HashMap, env};

use dotenvy::dotenv;
use lazy_static::lazy_static;
use provider::{
    BlockingProvider, EthersProvider, EthersProviderFactory, EvmBlockHeader,
    ProviderFactory,
};

fn get_alchemy_key() -> String {
    dotenv().ok();
    env::var("ALCHEMY_KEY").expect(
        "To use recording provider you need to set ALCHEMY_KEY in an .env file. See .env.example",
    )
}

lazy_static! {
    static ref alchemy_key: String = get_alchemy_key();
    static ref mainnet_url: String =
        format!("https://eth-mainnet.g.alchemy.com/v2/{}", *alchemy_key);
}

/// Performs a binary search to find the first block where `timestamp >= target_timestamp`.
/// Searches in the range `[start_block, end_block]`.
fn find_block_by_timestamp(
    provider: &Box<dyn BlockingProvider>, // Adjust trait bound as needed
    target_timestamp: u64,
    mut start_block: u64,
    mut end_block: u64,
) -> u64 {
    while start_block < end_block {
        let mid_block = (start_block + end_block) / 2;
        let block = provider
            .get_block_header(mid_block.into())
            .unwrap()
            .unwrap();

        if block.timestamp() < target_timestamp {
            start_block = mid_block + 1;
        } else {
            end_block = mid_block;
        }
    }
    start_block
}

/// Finds the first and last blocks within a given timestamp range by running two parallel binary searches.
/// Returns the block headers of the found lower and upper bound blocks.
pub fn find_block_range_by_timestamp(
    timestamp_start: u64,
    timestamp_end: u64,
) -> (Box<dyn EvmBlockHeader>, Box<dyn EvmBlockHeader>) {
    // Initialize the provider
    let rpc_endpoints = HashMap::from([(1, mainnet_url.clone())]);
    let provider_factory = EthersProviderFactory::new(rpc_endpoints);
    let provider = provider_factory.create(1).unwrap();

    let latest_block_number = provider.get_latest_block_number().ok().unwrap();

    // 1. Find the first block with timestamp >= timestamp_start (lower bound).
    let lower_block_number = find_block_by_timestamp(&provider, timestamp_start, 0, latest_block_number);

    // 2. Find the first block with timestamp > timestamp_end, then adjust to get upper bound.
    let upper_block_candidate = find_block_by_timestamp(&provider, timestamp_end + 1, 0, latest_block_number);
    let upper_block_number = if upper_block_candidate > 0 {
        upper_block_candidate - 1
    } else {
        0
    };

    // Retrieve the block headers for the found block numbers.
    let lower_block = provider
        .get_block_header(lower_block_number.into())
        .unwrap()
        .unwrap();
    let upper_block = provider
        .get_block_header(upper_block_number.into())
        .unwrap()
        .unwrap();

    (lower_block, upper_block)
}

#[cfg(test)]
mod tests {
    use super::*;

    const FIRST_BLOCK_TIMESTAMP: u64 = 1740545000;

    #[tokio::test(flavor = "multi_thread")]
    async fn gets_block_with_timestamp_higher_than_timestamp_start() {
        let timestamp_start = FIRST_BLOCK_TIMESTAMP;
        let timestamp_end = FIRST_BLOCK_TIMESTAMP + 100;
        let (lower_block, _) = find_block_range_by_timestamp(timestamp_start, timestamp_end);
        assert!(lower_block.timestamp() >= timestamp_start);
    }

    #[tokio::test(flavor = "multi_thread")]
    async fn gets_block_with_timestamp_lower_than_timestamp_end() {
        let timestamp_start = FIRST_BLOCK_TIMESTAMP;
        let timestamp_end = FIRST_BLOCK_TIMESTAMP + 100;
        let (_, upper_block) = find_block_range_by_timestamp(timestamp_start, timestamp_end);
        assert!(upper_block.timestamp() <= timestamp_end);
    }
}

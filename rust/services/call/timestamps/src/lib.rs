use std::{collections::HashMap, env};

use derive_new::new;
use dotenvy::dotenv;
use lazy_static::lazy_static;
use provider::{BlockingProvider, EthersProviderFactory, EvmBlockHeader, ProviderFactory};

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

// The actual genesis block timestamp is Jul-30-2015 03:26:13 PM +UTC
// (https://etherscan.io/block/0) but timestamp stored on the blockchain is 0
const ACTUAL_GENESIS_BLOCK_TIMESTAMP: u64 = 1_438_269_973;
const STORED_GENESIS_BLOCK_TIMESTAMP: u64 = 0;

#[derive(Debug, new)]
pub struct BlockRange {
    pub predecessor: Option<Box<dyn EvmBlockHeader>>,
    pub lower_block: Box<dyn EvmBlockHeader>,
    pub upper_block: Box<dyn EvmBlockHeader>,
    pub successor: Option<Box<dyn EvmBlockHeader>>,
}

/// Finds the first and last blocks within a given timestamp range by running two parallel binary searches.
pub fn find_block_range_by_timestamp(
    mut timestamp_start: u64,
    timestamp_end: u64,
    latest_block_number: u64,
) -> BlockRange {
    let provider = setup_provider();

    if timestamp_start <= ACTUAL_GENESIS_BLOCK_TIMESTAMP {
        timestamp_start = STORED_GENESIS_BLOCK_TIMESTAMP
    };

    let lower_block_number =
        find_block_by_timestamp(&provider, timestamp_start, 0, latest_block_number);

    let upper_block_candidate =
        find_block_by_timestamp(&provider, timestamp_end + 1, 0, latest_block_number);
    let upper_block_number = if upper_block_candidate > 0 {
        upper_block_candidate - 1
    } else {
        0
    };

    let lower_block = provider
        .get_block_header(lower_block_number.into())
        .unwrap()
        .unwrap();
    let upper_block = if lower_block_number == upper_block_number {
        lower_block.clone()
    } else {
        provider
            .get_block_header(upper_block_number.into())
            .unwrap()
            .unwrap()
    };

    let predecessor = get_predecessor(&provider, &*lower_block);
    let successor = get_successor(&provider, &*upper_block, latest_block_number);

    BlockRange::new(predecessor, lower_block, upper_block, successor)
}

fn setup_provider() -> Box<dyn BlockingProvider> {
    let rpc_endpoints = HashMap::from([(1, mainnet_url.clone())]);
    let provider_factory = EthersProviderFactory::new(rpc_endpoints);
    provider_factory.create(1).unwrap()
}

/// Performs a binary search to find the first block where `timestamp >= target_timestamp`.
/// Searches in the range `[start_block, end_block]`.
fn find_block_by_timestamp(
    provider: &dyn BlockingProvider,
    target_timestamp: u64,
    mut start_block: u64,
    mut end_block: u64,
) -> u64 {
    while start_block < end_block {
        let mid_block = (start_block + end_block) / 2;
        dbg!(mid_block);
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

fn get_predecessor(
    provider: &dyn BlockingProvider,
    block: &dyn EvmBlockHeader,
) -> Option<Box<dyn EvmBlockHeader>> {
    if block.number() > 0 {
        provider
            .get_block_header((block.number() - 1).into())
            .unwrap()
    } else {
        None
    }
}

fn get_successor(
    provider: &dyn BlockingProvider,
    block: &dyn EvmBlockHeader,
    latest_block_number: u64,
) -> Option<Box<dyn EvmBlockHeader>> {
    if block.number() < latest_block_number {
        provider
            .get_block_header((block.number() + 1).into())
            .unwrap()
    } else {
        None
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // https://etherscan.io/block/0
    const GENESIS_BLOCK_TIMESTAMP: u64 = 1_438_269_973;

    lazy_static! {
        static ref provider: Box<dyn BlockingProvider> = setup_provider();
    }

    mod find_block_range_by_timestamp {
        use super::*;

        #[tokio::test(flavor = "multi_thread")]
        async fn finds_block_range_by_timestamp() {
            let timestamp_start = GENESIS_BLOCK_TIMESTAMP;
            let timestamp_end = GENESIS_BLOCK_TIMESTAMP + 100;
            let latest_block_number = 1_000;

            let block_range =
                find_block_range_by_timestamp(timestamp_start, timestamp_end, latest_block_number);

            dbg!(&block_range.lower_block.timestamp());
            // dbg!(&block_range.predecessor.unwrap().timestamp());

            assert!(block_range.lower_block.timestamp() == 0);
            assert!(block_range.upper_block.timestamp() <= timestamp_end);
            assert!(block_range.predecessor.is_none());
            assert!(block_range.successor.unwrap().timestamp() > timestamp_end);
        }
    }

    #[tokio::test(flavor = "multi_thread")]
    async fn gets_block_with_timestamp_higher_than_timestamp_start() {
        let timestamp_start = GENESIS_BLOCK_TIMESTAMP;
        let timestamp_end = GENESIS_BLOCK_TIMESTAMP + 100;
        let latest_block_number = provider.get_latest_block_number().ok().unwrap();

        let block_range =
            find_block_range_by_timestamp(timestamp_start, timestamp_end, latest_block_number);
        let lower_block = block_range.lower_block;
        assert!(lower_block.timestamp() >= timestamp_start);

        if let Ok(Some(prev_block)) = provider.get_block_header((lower_block.number() - 1).into()) {
            assert!(prev_block.timestamp() < timestamp_start);
        }
    }

    #[tokio::test(flavor = "multi_thread")]
    async fn gets_block_with_timestamp_lower_than_timestamp_end() {
        let timestamp_start = GENESIS_BLOCK_TIMESTAMP;
        let timestamp_end = GENESIS_BLOCK_TIMESTAMP + 100;
        let latest_block_number = provider.get_latest_block_number().ok().unwrap();

        let block_range =
            find_block_range_by_timestamp(timestamp_start, timestamp_end, latest_block_number);
        let upper_block = block_range.upper_block;
        assert!(upper_block.timestamp() <= timestamp_end);

        if let Ok(Some(next_block)) = provider.get_block_header((upper_block.number() + 1).into()) {
            assert!(next_block.timestamp() > timestamp_end);
        }
    }
}

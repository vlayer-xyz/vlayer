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
    static ref ethers_provider: Box<dyn BlockingProvider> = setup_provider();
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

pub fn find_block_range_by_timestamp(
    mut timestamp_start: u64,
    timestamp_end: u64,
    latest_block_number: u64,
) -> BlockRange {
    if timestamp_start > timestamp_end {
        panic!("timestamp_start should be less than or equal to timestamp_end");
    }

    if timestamp_start <= ACTUAL_GENESIS_BLOCK_TIMESTAMP {
        timestamp_start = STORED_GENESIS_BLOCK_TIMESTAMP
    };

    let lower_block_number =
        find_first_block_ge_timestamp(&*ethers_provider, timestamp_start, 0, latest_block_number);
    let upper_block_number =
        find_last_block_le_timestamp(&*ethers_provider, timestamp_end, 0, latest_block_number);
    let lower_block = ethers_provider
        .get_block_header(lower_block_number.into())
        .unwrap()
        .unwrap();
    let upper_block = if lower_block_number == upper_block_number {
        lower_block.clone()
    } else {
        ethers_provider
            .get_block_header(upper_block_number.into())
            .unwrap()
            .unwrap()
    };

    let predecessor = get_predecessor(&*ethers_provider, &*lower_block);
    let successor = get_successor(&*ethers_provider, &*upper_block, latest_block_number);
    BlockRange::new(predecessor, lower_block, upper_block, successor)
}

fn setup_provider() -> Box<dyn BlockingProvider> {
    let rpc_endpoints = HashMap::from([(1, mainnet_url.clone())]);
    let provider_factory = EthersProviderFactory::new(rpc_endpoints);
    provider_factory.create(1).unwrap()
}

fn find_first_block_ge_timestamp(
    provider: &dyn BlockingProvider,
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

fn find_last_block_le_timestamp(
    provider: &dyn BlockingProvider,
    target_timestamp: u64,
    mut start_block: u64,
    mut end_block: u64,
) -> u64 {
    let mut result = start_block;
    while start_block <= end_block {
        let mid_block = (start_block + end_block) / 2;
        let block = provider
            .get_block_header(mid_block.into())
            .unwrap()
            .unwrap();
        if block.timestamp() <= target_timestamp {
            result = mid_block;
            start_block = mid_block + 1;
        } else {
            end_block = mid_block - 1;
        }
    }
    result
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

    lazy_static! {
        static ref provider: Box<dyn BlockingProvider> = setup_provider();
    }

    // https://etherscan.io/block/1000
    const BLOCK_ONE_THOUSAND_TIMESTAMP: u64 = 1_438_272_138;
    const LATEST_BLOCK_NUMBER: u64 = 1_000;

    // Tests ignored due to CI failing with missing ALCHEMY_KEY error
    mod find_block_range_by_timestamp {
        use super::*;

        #[tokio::test(flavor = "multi_thread")]
        #[ignore]
        async fn from_genesis() {
            let timestamp_start = ACTUAL_GENESIS_BLOCK_TIMESTAMP;
            let timestamp_end = ACTUAL_GENESIS_BLOCK_TIMESTAMP + 1000;

            let block_range =
                find_block_range_by_timestamp(timestamp_start, timestamp_end, LATEST_BLOCK_NUMBER);

            assert!(block_range.lower_block.timestamp() == 0);
            assert!(block_range.upper_block.timestamp() <= timestamp_end);
            assert!(block_range.predecessor.is_none());
            assert!(block_range.successor.unwrap().timestamp() > timestamp_end);
        }

        #[tokio::test(flavor = "multi_thread")]
        #[ignore]
        async fn until_last_block() {
            let timestamp_start = ACTUAL_GENESIS_BLOCK_TIMESTAMP + 1;

            let block_range = find_block_range_by_timestamp(
                timestamp_start,
                BLOCK_ONE_THOUSAND_TIMESTAMP,
                LATEST_BLOCK_NUMBER,
            );

            assert!(block_range.lower_block.timestamp() >= timestamp_start);
            assert!(block_range.upper_block.timestamp() <= BLOCK_ONE_THOUSAND_TIMESTAMP);
            assert!(block_range.predecessor.unwrap().timestamp() < timestamp_start);
            assert!(block_range.successor.is_none());
        }

        #[tokio::test(flavor = "multi_thread")]
        #[ignore]
        async fn intermediate_timestamps() {
            let timestamp_start = ACTUAL_GENESIS_BLOCK_TIMESTAMP + 1;
            let timestamp_end = BLOCK_ONE_THOUSAND_TIMESTAMP - 1;

            let block_range =
                find_block_range_by_timestamp(timestamp_start, timestamp_end, LATEST_BLOCK_NUMBER);

            assert!(block_range.lower_block.timestamp() >= timestamp_start);
            assert!(block_range.upper_block.timestamp() <= timestamp_end);
            assert!(block_range.predecessor.unwrap().timestamp() < timestamp_start);
            assert!(block_range.successor.unwrap().timestamp() > timestamp_end);
        }

        #[tokio::test(flavor = "multi_thread")]
        #[ignore]
        #[should_panic(expected = "timestamp_start should be less than or equal to timestamp_end")]
        async fn panics_if_timestamp_start_is_greater_than_timestamp_end() {
            let timestamp_start = BLOCK_ONE_THOUSAND_TIMESTAMP;
            let timestamp_end = ACTUAL_GENESIS_BLOCK_TIMESTAMP;

            find_block_range_by_timestamp(timestamp_start, timestamp_end, LATEST_BLOCK_NUMBER);
        }
    }
}

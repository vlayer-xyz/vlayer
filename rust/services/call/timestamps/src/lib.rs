use anyhow::Error;
use provider::{BlockingProvider, EvmBlockHeader};

// The actual genesis block timestamp is Jul-30-2015 03:26:13 PM +UTC
// (https://etherscan.io/block/0) but timestamp stored on the blockchain is 0
const ACTUAL_MAINNET_GENESIS_BLOCK_TIMESTAMP: u64 = 1_438_269_973;

#[derive(Debug)]
pub struct BlockRange {
    pub start: u64,
    pub end: u64,
}

impl TryFrom<(u64, u64)> for BlockRange {
    type Error = Error;

    fn try_from(value: (u64, u64)) -> Result<Self, Self::Error> {
        let (start, end) = value;
        if start > end {
            return Err(anyhow::anyhow!(
                "Start block ({}) must be less than or equal to end block ({})",
                start,
                end
            ));
        }

        Ok(BlockRange { start, end })
    }
}

pub fn find_first_block_ge_timestamp(
    provider: &dyn BlockingProvider,
    target_timestamp: u64,
    range: BlockRange,
) -> Option<Box<dyn EvmBlockHeader>> {
    if target_timestamp <= ACTUAL_MAINNET_GENESIS_BLOCK_TIMESTAMP {
        return provider.get_block_header(0.into()).unwrap();
    }

    let block_number = binary_search_block_number(provider, target_timestamp, range);

    block_number.and_then(|number| provider.get_block_header(number.into()).unwrap())
}

/// Searches for the earliest block within the given range that has a timestamp
/// greater than or equal to the target timestamp using binary search.
/// If no such block is found (i.e., all blocks in the range have timestamps below the target),
/// the function returns `None`.
pub fn binary_search_block_number(
    provider: &dyn BlockingProvider,
    target_timestamp: u64,
    mut range: BlockRange,
) -> Option<u64> {
    while range.start < range.end {
        let mid_block = (range.start + range.end) / 2;
        let block = provider
            .get_block_header(mid_block.into())
            .expect("Failed to fetch block")
            .unwrap();

        if block.timestamp() < target_timestamp {
            range.start = mid_block + 1;
        } else {
            range.end = mid_block;
        }
    }

    // Loop above can return a block with timestamp < target_timestamp if it is the last block
    provider
        .get_block_header(range.start.into())
        .expect("Failed to fetch block")
        .and_then(|b| {
            if b.timestamp() < target_timestamp {
                None
            } else {
                Some(b.number())
            }
        })
}

#[cfg(test)]
mod tests {
    use std::{collections::HashMap, env};

    use ::provider::{EthersProviderFactory, ProviderFactory};
    use dotenvy::dotenv;
    use lazy_static::lazy_static;

    use super::*;

    lazy_static! {
        static ref mainnet_url: String = get_mainnet_url();
        static ref provider: Box<dyn BlockingProvider> = setup_provider();
    }

    // https://etherscan.io/block/20000000
    const LATEST_BLOCK_NUMBER: u64 = 20_000_000;
    const LATEST_BLOCK_TIMESTAMP: u64 = 1_717_281_407;

    fn get_mainnet_url() -> String {
        dotenv().ok();
        env::var("MAINNET_URL")
            .expect("To use provider you need to set MAINNET_URL in an .env file.")
    }

    fn setup_provider() -> Box<dyn BlockingProvider> {
        let rpc_endpoints = HashMap::from([(1, mainnet_url.clone())]);
        let provider_factory = EthersProviderFactory::new(rpc_endpoints);
        provider_factory.create(1).unwrap()
    }

    // Tests ignored because network connection necessary to run them is not possible on CI
    // todo: Add snapshot mechanism to run these tests
    mod find_first_block_ge_timestamp {
        use super::*;

        #[tokio::test(flavor = "multi_thread")]
        #[ignore]
        async fn genesis_case() -> anyhow::Result<()> {
            let block_range = (0, LATEST_BLOCK_NUMBER).try_into()?;
            let target_timestamp = ACTUAL_MAINNET_GENESIS_BLOCK_TIMESTAMP;

            let block =
                find_first_block_ge_timestamp(&*provider, target_timestamp, block_range).unwrap();

            assert!(block.number() == 0);

            Ok(())
        }

        #[tokio::test(flavor = "multi_thread")]
        #[ignore]
        async fn found() -> anyhow::Result<()> {
            let block_range = (0, LATEST_BLOCK_NUMBER).try_into()?;
            let target_timestamp = LATEST_BLOCK_TIMESTAMP;

            let block =
                find_first_block_ge_timestamp(&*provider, target_timestamp, block_range).unwrap();

            assert!(block.timestamp() >= target_timestamp);

            let previous_block = provider
                .get_block_header((block.number() - 1).into())
                .unwrap()
                .unwrap();
            assert!(previous_block.timestamp() < target_timestamp);

            Ok(())
        }
    }

    mod binary_search_block_number {
        use super::*;

        #[tokio::test(flavor = "multi_thread")]
        #[ignore]
        async fn found() -> anyhow::Result<()> {
            let block_range = (0, LATEST_BLOCK_NUMBER).try_into()?;
            let target_timestamp =
                (ACTUAL_MAINNET_GENESIS_BLOCK_TIMESTAMP + LATEST_BLOCK_TIMESTAMP) / 2;

            let block_number =
                binary_search_block_number(&*provider, target_timestamp, block_range);

            let block = provider
                .get_block_header(block_number.unwrap().into())?
                .unwrap();
            let previous_block = provider
                .get_block_header((block.number() - 1).into())?
                .unwrap();

            assert!(block.timestamp() >= target_timestamp);
            assert!(previous_block.timestamp() < target_timestamp);

            Ok(())
        }

        #[tokio::test(flavor = "multi_thread")]
        #[ignore]
        async fn not_found() -> anyhow::Result<()> {
            let block_range = (0, LATEST_BLOCK_NUMBER).try_into()?;
            let target_timestamp = LATEST_BLOCK_TIMESTAMP + 1;

            let block = binary_search_block_number(&*provider, target_timestamp, block_range);

            assert!(block.is_none());

            Ok(())
        }

        #[tokio::test(flavor = "multi_thread")]
        #[ignore]
        async fn timestamp_too_big() -> anyhow::Result<()> {
            let block_range = (0, LATEST_BLOCK_NUMBER).try_into()?;
            let target_timestamp = LATEST_BLOCK_TIMESTAMP + 1;

            let block = binary_search_block_number(&*provider, target_timestamp, block_range);

            assert!(block.is_none());

            Ok(())
        }
    }
}

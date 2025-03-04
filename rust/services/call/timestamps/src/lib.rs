use provider::{BlockingProvider, EvmBlockHeader};
use thiserror::Error;
use u64_range::NonEmptyRange;

type BlockRange = NonEmptyRange;

#[derive(Debug, Error)]
pub enum Error {
    #[error("Block not found")]
    BlockNotFound,
    #[error("Provider error")]
    ProviderError(#[from] provider::Error),
    #[error("Minimal timestamp to be used is {0}")]
    TimestampTooEarly(u64),
}

type BlockPair = (Box<dyn EvmBlockHeader>, Box<dyn EvmBlockHeader>);

/// `left_block` has timestamp lower than `target_timestamp`.
/// `right_block` has timestamp greater or equal to `target_timestamp`.
pub fn find_blocks_by_timestamp(
    range: BlockRange,
    target_timestamp: u64,
    provider: &dyn BlockingProvider,
) -> Result<BlockPair, Error> {
    // Block 0 has a timestamp equal to 0. To avoid handling that timestamp case,
    // this function only works with blocks with number 1 or higher.
    let minimal_timestamp = get_block_timestamp(1, provider)?;
    if target_timestamp < minimal_timestamp {
        return Err(Error::TimestampTooEarly(minimal_timestamp));
    }

    let block_number = range
        .find_ge(target_timestamp, |block_number| get_block_timestamp(block_number, provider))?;
    let block_number = block_number.ok_or(Error::BlockNotFound)?;
    let left_block = get_block(block_number - 1, provider)?;
    let right_block = get_block(block_number, provider)?;

    Ok((left_block, right_block))
}

fn get_block(
    block_number: u64,
    provider: &dyn BlockingProvider,
) -> Result<Box<dyn EvmBlockHeader>, Error> {
    match provider.get_block_header(block_number.into()) {
        Ok(Some(block)) => Ok(block),
        Ok(None) => Err(Error::BlockNotFound),
        Err(e) => Err(Error::ProviderError(e)),
    }
}

fn get_block_timestamp(block_number: u64, provider: &dyn BlockingProvider) -> Result<u64, Error> {
    Ok(get_block(block_number, provider)?.timestamp())
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
    const FIRST_BLOCK_TIMESTAMP: u64 = 1_438_269_988;
    const NON_EXISTING_BLOCK: u64 = 1_000_000_000_000_000;

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
    mod find_blocks_by_timestamp {
        use super::*;

        #[tokio::test(flavor = "multi_thread")]
        #[ignore]
        async fn timestamp_too_early() -> anyhow::Result<()> {
            let block_range = (0..=0).try_into()?;
            let target_timestamp = 0;

            let result = find_blocks_by_timestamp(block_range, target_timestamp, &*provider);

            assert!(matches!(result, Err(Error::TimestampTooEarly(FIRST_BLOCK_TIMESTAMP))));

            Ok(())
        }

        #[tokio::test(flavor = "multi_thread")]
        #[ignore]
        async fn success() -> anyhow::Result<()> {
            let block_range = (0..=LATEST_BLOCK_NUMBER).try_into()?;
            let target_timestamp = LATEST_BLOCK_TIMESTAMP;

            let (left_block, right_block) =
                find_blocks_by_timestamp(block_range, target_timestamp, &*provider).unwrap();

            assert!(left_block.timestamp() < target_timestamp);
            assert!(right_block.timestamp() >= target_timestamp);

            Ok(())
        }
    }

    #[cfg(test)]
    mod get_block {
        use super::*;

        #[tokio::test(flavor = "multi_thread")]
        #[ignore]
        async fn success() -> anyhow::Result<()> {
            let block_number = 1;
            let block = get_block(block_number, &*provider)?;

            assert_eq!(block.number(), block_number);

            Ok(())
        }

        #[tokio::test(flavor = "multi_thread")]
        #[ignore]
        async fn block_not_found() -> anyhow::Result<()> {
            let result = get_block(NON_EXISTING_BLOCK, &*provider);

            assert!(matches!(result, Err(Error::BlockNotFound)));

            Ok(())
        }
    }
}

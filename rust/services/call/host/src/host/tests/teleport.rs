use std::time::{Duration, SystemTime};

use alloy_chains::{Chain, NamedChain};
use alloy_primitives::{ChainId, b256};
use call_common::{ExecutionLocation, RevmDB};
use call_db::ProviderDb;
use call_engine::verifier::teleport::fetch_latest_confirmed_l2_block;
use ethers_core::types::U64;
use jsonrpsee::http_client::HttpClientBuilder;
use optimism::{
    IClient, NumHash,
    anchor_state_registry::{AnchorStateRegistry, L2Commitment},
    client::http,
    types::SequencerOutput,
};
use provider::{BlockTag, EthersProviderFactory, ProviderFactory};

use crate::test_harness::rpc::{quicknode_op_sepolia_url, rpc_urls};

fn get_db(location: ExecutionLocation) -> anyhow::Result<impl RevmDB> {
    let provider_factory = EthersProviderFactory::new(rpc_urls());
    let source_provider = provider_factory.create(location.chain_id)?;
    let db = ProviderDb::new(source_provider.into(), location.block_number);
    Ok(db)
}

fn create_anchor_state_registry(
    location: ExecutionLocation,
    dest_chain_id: ChainId,
) -> anyhow::Result<AnchorStateRegistry<impl RevmDB>> {
    let db = get_db(location)?;

    let dest_chain_spec = chain::optimism::ChainSpec::try_from(dest_chain_id)?;
    dest_chain_spec.assert_anchor(location.chain_id)?;

    let registry = AnchorStateRegistry::new(dest_chain_spec.anchor_state_registry, db);
    Ok(registry)
}

fn check_anchor_state_freshness(
    src_chain: Chain,
    dest_chain: Chain,
    max_age_hours: u64,
) -> anyhow::Result<()> {
    let factory = EthersProviderFactory::new(rpc_urls());
    let src = factory.create(src_chain.id())?;
    let dest = factory.create(dest_chain.id())?;

    let current_block = src.get_latest_block_number()?;
    let registry =
        create_anchor_state_registry((src_chain.id(), current_block).into(), dest_chain.id())?;
    let commitment = registry.get_latest_confirmed_l2_commitment()?;

    let Some(block) =
        dest.get_block_header(BlockTag::Number(U64::from(commitment.block_number)))?
    else {
        anyhow::bail!("No block found for number {}", commitment.block_number)
    };

    ensure_block_fresh(block.timestamp(), max_age_hours, src_chain, dest_chain)?;
    Ok(())
}

fn ensure_block_fresh(
    ts_secs: u64,
    max_age_hours: u64,
    src_chain: Chain,
    dest_chain: Chain,
) -> anyhow::Result<()> {
    let block_time = SystemTime::UNIX_EPOCH + Duration::from_secs(ts_secs);
    let block_age = SystemTime::now().duration_since(block_time)?;
    let max_age = Duration::from_secs(3600 * max_age_hours);
    let age_hours = block_age.as_secs_f64() / 3600.0;

    anyhow::ensure!(
        block_age <= max_age,
        "Latest finalized block for {} -> {} is too old: {:.2}h (max {}h)",
        src_chain.named().unwrap(),
        dest_chain.named().unwrap(),
        age_hours,
        max_age_hours,
    );
    Ok(())
}

mod anchor_state_registry {
    use super::*;

    macro_rules! anchor_state_registry_test {
        (
            $test_name:ident,
            src = $src_chain:expr,
            dest = $dest_chain:expr,
            height = $height:literal,
            hash = $hash:literal,
            block = $block:literal
        ) => {
            #[tokio::test(flavor = "multi_thread")]
            #[ignore = "This test is ignored because it requires alchemy Api key set in .env file"]
            async fn $test_name() -> anyhow::Result<()> {
                let anchor_state_registry = create_anchor_state_registry(
                    ($src_chain.id(), $height).into(),
                    $dest_chain.id(),
                )?;
                let l2_commitment = anchor_state_registry.get_latest_confirmed_l2_commitment()?;

                assert_eq!(l2_commitment, L2Commitment::new(b256!($hash), $block));

                Ok(())
            }
        };
    }
    mod op_sepolia {
        use super::*;
        anchor_state_registry_test!(
            newer,
            src = Chain::sepolia(),
            dest = Chain::optimism_sepolia(),
            height = 8_178_125_u64,
            hash = "f72a99833d9b110fdab1e624117d9b4b03a09a2904ad50aa1634946a8740e542",
            block = 26_494_775
        );

        anchor_state_registry_test!(
            older,
            src = Chain::sepolia(),
            dest = Chain::optimism_sepolia(),
            height = 8_108_125_u64,
            hash = "f1b2a502390f65d87f4966f2767fe345233100bb92707adf616f5e83f6bd4a4a",
            block = 26_054_710
        );
    }

    mod base_sepolia {
        use super::*;
        anchor_state_registry_test!(
            some,
            src = Chain::sepolia(),
            dest = Chain::base_sepolia(),
            height = 8_178_125_u64,
            hash = "499579439ec7e1ac9b93f5a001f4608777f1b6a776938b2b013fa4116857ee96",
            block = 24_510_566
        );
    }

    mod worldchain_sepolia {
        use super::*;
        anchor_state_registry_test!(
            some,
            src = Chain::sepolia(),
            dest = Chain::from_named(NamedChain::WorldSepolia),
            height = 8_178_125_u64,
            hash = "c5df8df8ef8c5164b3249bbf67db9c3dbb4f584a00c3072399629c088d57ac75",
            block = 12_271_778
        );
    }

    mod unichain_sepolia {
        use super::*;
        anchor_state_registry_test!(
            some,
            src = Chain::sepolia(),
            dest = Chain::from_named(NamedChain::UnichainSepolia),
            height = 8_178_125_u64,
            hash = "34184ca6e9f0d6f2c3c30e953b40981bab5faee02ae249d026ecea7250651703",
            block = 16_681_148
        );
    }

    mod freshness {
        use super::*;

        const MAX_AGE_HOURS: u64 = 170;

        mod sepolia {
            use super::*;

            #[tokio::test(flavor = "multi_thread")]
            #[ignore = "This test is ignored because it requires alchemy Api key set in .env file"]
            async fn optimism() -> anyhow::Result<()> {
                check_anchor_state_freshness(
                    Chain::sepolia(),
                    Chain::optimism_sepolia(),
                    MAX_AGE_HOURS,
                )
            }

            #[tokio::test(flavor = "multi_thread")]
            #[ignore = "This test is ignored because it requires alchemy Api key set in .env file"]
            async fn base() -> anyhow::Result<()> {
                check_anchor_state_freshness(Chain::sepolia(), Chain::base_sepolia(), MAX_AGE_HOURS)
            }

            #[tokio::test(flavor = "multi_thread")]
            #[ignore = "This test is ignored because it requires alchemy Api key set in .env file"]
            async fn world() -> anyhow::Result<()> {
                check_anchor_state_freshness(
                    Chain::sepolia(),
                    Chain::from_named(NamedChain::WorldSepolia),
                    MAX_AGE_HOURS,
                )
            }

            #[tokio::test(flavor = "multi_thread")]
            #[ignore = "This test is ignored because AnchorStateRegistry is not updated on unichain docs"]
            async fn unichain() -> anyhow::Result<()> {
                check_anchor_state_freshness(
                    Chain::sepolia(),
                    Chain::from_named(NamedChain::UnichainSepolia),
                    MAX_AGE_HOURS,
                )
            }
        }

        mod mainnet {
            use super::*;

            #[tokio::test(flavor = "multi_thread")]
            #[ignore = "This test is ignored because it requires alchemy Api key set in .env file"]
            async fn optimism() -> anyhow::Result<()> {
                check_anchor_state_freshness(
                    Chain::mainnet(),
                    Chain::optimism_mainnet(),
                    MAX_AGE_HOURS,
                )
            }

            #[tokio::test(flavor = "multi_thread")]
            #[ignore = "This test is ignored because it requires alchemy Api key set in .env file"]
            async fn base() -> anyhow::Result<()> {
                check_anchor_state_freshness(Chain::mainnet(), Chain::base_mainnet(), MAX_AGE_HOURS)
            }

            #[tokio::test(flavor = "multi_thread")]
            #[ignore = "This test is ignored because it requires alchemy Api key set in .env file"]
            async fn world() -> anyhow::Result<()> {
                check_anchor_state_freshness(
                    Chain::mainnet(),
                    Chain::from_named(NamedChain::World),
                    MAX_AGE_HOURS,
                )
            }

            #[tokio::test(flavor = "multi_thread")]
            #[ignore = "This test is ignored because it requires alchemy Api key set in .env file"]
            async fn unichain() -> anyhow::Result<()> {
                check_anchor_state_freshness(
                    Chain::mainnet(),
                    Chain::from_named(NamedChain::Unichain),
                    MAX_AGE_HOURS,
                )
            }
        }
    }
}

mod sequencer_client {
    use super::*;

    #[tokio::test(flavor = "multi_thread")]
    #[ignore = "This test is ignored because it requires quicknode Api key set in .env file"]
    async fn test_sequencer_client() -> anyhow::Result<()> {
        let http_client = HttpClientBuilder::default().build(&*quicknode_op_sepolia_url)?;
        let sequencer_client = http::Client::new(http_client);

        let output = sequencer_client.get_output_at_block(26_054_710).await?;

        assert_eq!(
            output,
            SequencerOutput {
                version: b256!("0000000000000000000000000000000000000000000000000000000000000000"),
                state_root: b256!(
                    "fb9c656ed8b9c26580a963decdfe15276a079cd9c464db4954f7116c2ad686cd"
                ),
                withdrawal_storage_root: b256!(
                    "4ff5de258db6b26d9f509b9d60ce7c9287e24c7f98d9f529409a80c51bc666e2"
                ),
                l2_block: NumHash {
                    number: 26_054_710_u64,
                    hash: b256!("9f5e995fb3f60e3fc0862d2a94f3ff3c92ffb85c33a1675c27a22bba675e69a3"),
                },
            }
        );

        Ok(())
    }
}

mod output_hash_match {
    use super::*;

    #[tokio::test(flavor = "multi_thread")]
    #[ignore = "This test is ignored because it requires quicknode Api key set in .env file"]
    async fn sepolia_op_sepolia() -> anyhow::Result<()> {
        let src_chain = Chain::sepolia();
        let dest_chain = Chain::optimism_sepolia();

        let http_client = HttpClientBuilder::default().build(&*quicknode_op_sepolia_url)?;
        let sequencer_client = http::Client::new(http_client);
        let anchor_state_registry =
            create_anchor_state_registry((src_chain.id(), 8_108_125_u64).into(), dest_chain.id())?;

        let comnfirmed_l2_block =
            fetch_latest_confirmed_l2_block(anchor_state_registry, &sequencer_client).await?;

        assert_eq!(
            comnfirmed_l2_block,
            NumHash {
                number: 26_054_710_u64,
                hash: b256!("9f5e995fb3f60e3fc0862d2a94f3ff3c92ffb85c33a1675c27a22bba675e69a3"),
            }
        );

        Ok(())
    }
}

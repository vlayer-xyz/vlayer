use std::{
    process,
    time::{Duration, SystemTime},
};

use alloy_chains::{Chain, NamedChain};
use alloy_primitives::ChainId;
use anyhow::{bail, ensure};
use call_common::{ExecutionLocation, RevmDB};
use call_db::ProviderDb;
use call_rpc::rpc_urls;
use chain::optimism::ChainSpec;
use optimism::anchor_state_registry::AnchorStateRegistry;
use provider::{BlockTag, EthersProviderFactory, EvmBlockHeader, ProviderFactory};

const MAX_AGE_HOURS: u64 = 170;
const BASE_MAX_AGE_HOURS: u64 = 80;

lazy_static::lazy_static! {
    static ref PROVIDER_FACTORY: EthersProviderFactory = EthersProviderFactory::new(rpc_urls());
}

fn get_db(location: ExecutionLocation) -> anyhow::Result<impl RevmDB> {
    let source_provider = PROVIDER_FACTORY.create(location.chain_id)?;
    let db = ProviderDb::new(source_provider.into(), location.block_number);
    Ok(db)
}

fn create_anchor_state_registry(
    location: ExecutionLocation,
    dest_chain_id: ChainId,
) -> anyhow::Result<AnchorStateRegistry<impl RevmDB>> {
    let db = get_db(location)?;
    let dest_chain_spec = ChainSpec::try_from(dest_chain_id)?;
    dest_chain_spec.assert_anchor(location.chain_id)?;
    let registry = AnchorStateRegistry::new(dest_chain_spec.anchor_state_registry, db);
    Ok(registry)
}

fn age_in_hours(block: &Box<dyn EvmBlockHeader>) -> anyhow::Result<f64> {
    let block_time = SystemTime::UNIX_EPOCH + Duration::from_secs(block.timestamp());
    let age = SystemTime::now().duration_since(block_time)?;
    Ok(age.as_secs_f64() / 3600.0)
}

fn check_anchor_state_liveliness(
    src_chain: Chain,
    dest_chain: Chain,
    max_age_seconds: u64,
) -> anyhow::Result<()> {
    let src = PROVIDER_FACTORY.create(src_chain.id())?;
    let dest = PROVIDER_FACTORY.create(dest_chain.id())?;
    let current_src_chain_block = src.get_latest_block_number()?;
    let registry = create_anchor_state_registry(
        (src_chain.id(), current_src_chain_block).into(),
        dest_chain.id(),
    )?;
    let commitment = registry.get_latest_confirmed_l2_commitment()?;
    let Some(current_dest_chain_block) =
        dest.get_block_header(BlockTag::Number(commitment.block_number.into()))?
    else {
        bail!(
            "Block {} on destination chain {} not found",
            commitment.block_number,
            dest_chain.named().unwrap(),
        )
    };
    let block_age_hours = age_in_hours(&current_dest_chain_block)?;
    let block_age_secs = (block_age_hours * 3600.0) as u64;
    let max_age_hours = max_age_seconds as f64 / 3600.0;

    ensure!(
        block_age_secs <= max_age_seconds,
        "Latest finalized block for {} -> {} is too old: {:.2}h (max {:.2}h)",
        src_chain.named().unwrap(),
        dest_chain.named().unwrap(),
        block_age_hours,
        max_age_hours,
    );
    Ok(())
}

#[tokio::main]
pub async fn main() -> anyhow::Result<()> {
    let src_chain_to_dest_chain = vec![
        (Chain::sepolia(), Chain::optimism_sepolia()),
        (Chain::sepolia(), Chain::base_sepolia()),
        (Chain::sepolia(), Chain::from_named(NamedChain::WorldSepolia)),
        (Chain::sepolia(), Chain::from_named(NamedChain::UnichainSepolia)),
        (Chain::mainnet(), Chain::optimism_mainnet()),
        (Chain::mainnet(), Chain::base_mainnet()),
        (Chain::mainnet(), Chain::from_named(NamedChain::World)),
        (Chain::mainnet(), Chain::from_named(NamedChain::Unichain)),
    ];
    let max_age_hours = 170;
    let max_age_seconds = max_age_hours * 3600;
    let mut any_failed = false;
    for (src, dest) in src_chain_to_dest_chain {
        print!(
            "Checking anchor liveliness for {} -> {}... ",
            src.named().unwrap(),
            dest.named().unwrap()
        );
        match check_anchor_state_liveliness(src, dest, max_age_seconds) {
            Ok(()) => println!("OK"),
            Err(e) => {
                println!("STALE or ERROR: {e}");
                any_failed = true;
            }
        }
    }
    if any_failed {
        process::exit(1);
    }
    Ok(())
}

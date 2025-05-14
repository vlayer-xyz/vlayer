use std::{
    collections::HashMap,
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

const ALLOWED_BLOCK_AGE_HOURS_DEVIATION: u64 = 5;

// hours
const SEVEN_DAYS: u64 = 7 * 24;
const THREE_AND_A_HALF_DAYS: u64 = 3 * 24 + 12;

lazy_static::lazy_static! {
    static ref PROVIDER_FACTORY: EthersProviderFactory = EthersProviderFactory::new(rpc_urls());
    static ref CHAIN_TO_FINALISATION_TIME: HashMap<u64, u64> = HashMap::from([
        (Chain::optimism_sepolia().id(), SEVEN_DAYS),
        (Chain::base_sepolia().id(), THREE_AND_A_HALF_DAYS),
        (Chain::from_named(NamedChain::WorldSepolia).id(), THREE_AND_A_HALF_DAYS),
        (Chain::from_named(NamedChain::UnichainSepolia).id(), SEVEN_DAYS),
        (Chain::optimism_mainnet().id(), SEVEN_DAYS),
        (Chain::base_mainnet().id(), THREE_AND_A_HALF_DAYS),
        (Chain::from_named(NamedChain::World).id(), THREE_AND_A_HALF_DAYS),
        (Chain::from_named(NamedChain::Unichain).id(), SEVEN_DAYS),
    ]);
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

#[allow(clippy::borrowed_box)]
fn age_in_hours(block: &Box<dyn EvmBlockHeader>) -> anyhow::Result<f64> {
    let block_time = SystemTime::UNIX_EPOCH + Duration::from_secs(block.timestamp());
    let age = SystemTime::now().duration_since(block_time)?;
    Ok(age.as_secs_f64() / 3600.0)
}

fn expected_block_age_hours(dest_chain: &Chain) -> u64 {
    #[allow(clippy::unwrap_used)]
    *CHAIN_TO_FINALISATION_TIME.get(&dest_chain.id()).unwrap()
}

#[allow(clippy::cast_precision_loss)]
fn is_within_expected_block_age(
    block_age_hours: f64,
    expected_hours: u64,
    allowed_deviation: u64,
) -> bool {
    (expected_hours as f64 - allowed_deviation as f64) <= block_age_hours
        && block_age_hours <= (expected_hours as f64 + allowed_deviation as f64)
}

fn check_anchor_state_liveliness(src_chain: Chain, dest_chain: Chain) -> anyhow::Result<()> {
    let src = PROVIDER_FACTORY.create(src_chain.id())?;
    let dest = PROVIDER_FACTORY.create(dest_chain.id())?;
    let current_block = src.get_latest_block_number()?;
    let registry =
        create_anchor_state_registry((src_chain.id(), current_block).into(), dest_chain.id())?;
    let commitment = registry.get_latest_confirmed_l2_commitment()?;

    #[allow(clippy::unwrap_used)]
    let src_name = src_chain.named().unwrap();
    #[allow(clippy::unwrap_used)]
    let dest_name = dest_chain.named().unwrap();

    let Some(current_dest_chain_block) =
        dest.get_block_header(BlockTag::Number(commitment.block_number.into()))?
    else {
        bail!("No block {} on destination chain {dest_name}", commitment.block_number)
    };
    let block_age_hours = age_in_hours(&current_dest_chain_block)?;
    let expected_hours = expected_block_age_hours(&dest_chain);
    ensure!(
        is_within_expected_block_age(
            block_age_hours,
            expected_hours,
            ALLOWED_BLOCK_AGE_HOURS_DEVIATION
        ),
        "Block for {src_name} -> {dest_name} is not within expected age range: {:.2}h (expected: {}±{}h)",
        block_age_hours,
        expected_hours,
        ALLOWED_BLOCK_AGE_HOURS_DEVIATION,
    );
    Ok(())
}

#[tokio::main]
#[allow(clippy::unwrap_used)]
pub async fn main() -> anyhow::Result<()> {
    let src_chain_to_dest_chain = vec![
        (Chain::sepolia(), Chain::optimism_sepolia()),
        (Chain::sepolia(), Chain::base_sepolia()),
        (Chain::sepolia(), Chain::from_named(NamedChain::WorldSepolia)),
        // Currently, we don't have an up-to-date `AnchorStateRegistry address for Unichain Sepolia.
        // (Chain::sepolia(), Chain::from_named(NamedChain::UnichainSepolia)),
        (Chain::mainnet(), Chain::optimism_mainnet()),
        (Chain::mainnet(), Chain::base_mainnet()),
        (Chain::mainnet(), Chain::from_named(NamedChain::World)),
        (Chain::mainnet(), Chain::from_named(NamedChain::Unichain)),
    ];
    let mut any_failed = false;
    for (src, dest) in src_chain_to_dest_chain {
        let src_name = src.named().unwrap();
        let dest_name = dest.named().unwrap();
        print!("Checking anchor liveliness for {src_name} -> {dest_name}... ");
        match check_anchor_state_liveliness(src, dest) {
            Ok(()) => println!("OK"),
            Err(e) => {
                println!("error: {e}");
                any_failed = true;
            }
        }
    }
    if any_failed {
        process::exit(1);
    }
    Ok(())
}

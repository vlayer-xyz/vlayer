use std::process;

use alloy_chains::{Chain, NamedChain};
use alloy_primitives::ChainId;
use call_common::{ExecutionLocation, RevmDB};
use call_db::ProviderDb;
use call_rpc::rpc_urls;
use chain::optimism::ChainSpec;
use optimism::anchor_state_registry::AnchorStateRegistry;
use provider::{BlockTag, EthersProviderFactory, ProviderFactory};

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
    let dest_chain_spec = ChainSpec::try_from(dest_chain_id)?;
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
    let Some(block) = dest.get_block_header(BlockTag::Number(commitment.block_number.into()))?
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
    use std::time::{Duration, SystemTime};
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
    let mut any_failed = false;
    for (src, dest) in src_chain_to_dest_chain {
        print!(
            "Checking anchor freshness for {} -> {}... ",
            src.named().unwrap(),
            dest.named().unwrap()
        );
        match check_anchor_state_freshness(src, dest, max_age_hours) {
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

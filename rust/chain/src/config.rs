//! Handling different blockchain specifications.
use std::collections::HashMap;

use alloy_chains::Chain;
use alloy_primitives::ChainId;
use once_cell::sync::Lazy;
use revm::primitives::SpecId::*;
use toml::de::Error;

use crate::{fork::Fork, spec::ChainSpec};

// Some unique chain ids for testing
pub const TEST_CHAIN_ID: ChainId = 31_337;

// https://etherscan.io/block/15537394
pub const MAINNET_MERGE_BLOCK_NUMBER: u64 = 15537394;
pub const MAINNET_MERGE_BLOCK_TIMESTAMP: u64 = 1663224179;

pub static CHAIN_MAP: Lazy<HashMap<ChainId, &'static Lazy<ChainSpec>>> = Lazy::new(|| {
    HashMap::from([
        (Chain::mainnet().id(), &ETH_MAINNET_CHAIN_SPEC),
        (Chain::sepolia().id(), &ETH_SEPOLIA_CHAIN_SPEC),
        (TEST_CHAIN_ID, &TESTING_CHAIN_SPEC),
        (Chain::base_mainnet().id(), &BASE_CHAIN_SPEC),
        (Chain::base_sepolia().id(), &BASE_SEPOLIA_CHAIN_SPEC),
        (Chain::optimism_mainnet().id(), &OP_MAINNET_CHAIN_SPEC),
        (Chain::optimism_sepolia().id(), &OP_SEPOLIA_CHAIN_SPEC),
        (137, &POLYGON_CHAIN_SPEC),
        (80002, &POLYGON_AMOY_CHAIN_SPEC),
        (42161, &ARBITRUM_ONE_CHAIN_SPEC),
        (42170, &ARBITRUM_NOVA_CHAIN_SPEC),
        (421614, &ARBITRUM_SEPOLIA_CHAIN_SPEC),
        (324, &ZKSYNC_CHAIN_SPEC),
        (300, &ZKSYNC_SEPOLIA_CHAIN_SPEC),
        (747, &FLOW_MAINNET_CHAIN_SPEC),
        (545, &FLOW_TESTNET_CHAIN_SPEC),
        (534352, &SCROLL_MAINNET_CHAIN_SPEC),
        (534351, &SCROLL_SEPOLIA_CHAIN_SPEC),
        (5000, &MANTLE_MAINNET_CHAIN_SPEC),
        (59144, &LINEA_MAINNET_CHAIN_SPEC),
        (59141, &LINEA_SEPOLIA_CHAIN_SPEC),
        (96, &BITKUB_CHAIN_CHAIN_SPEC),
        (25925, &BITKUB_CHAIN_TESTNET_CHAIN_SPEC),
        (7887, &KINTO_CHAIN_SPEC),
        (42220, &CELO_CHAIN_SPEC),
        (48900, &ZIRCUIT1_MAINNET_CHAIN_SPEC),
        (48899, &ZIRCUIT1_TESTNET_CHAIN_SPEC),
        (100, &GNOSIS_MAINNET_CHAIN_SPEC),
        (10200, &GNOSIS_CHIADO_CHAIN_SPEC),
        (8008135, &PHENIX_CHAIN_SPEC),
    ])
});

pub static CHAIN_NAME_TO_ID: Lazy<HashMap<String, ChainId>> = Lazy::new(|| {
    HashMap::from([
        ("mainnet".into(), Chain::mainnet().id()),
        ("sepolia".into(), Chain::sepolia().id()),
        ("base".into(), Chain::base_mainnet().id()),
        ("base-sepolia".into(), Chain::base_sepolia().id()),
        ("optimism".into(), Chain::optimism_mainnet().id()),
        ("optimism-sepolia".into(), Chain::optimism_sepolia().id()),
        ("polygon".into(), 137),
        ("polygon-amoy".into(), 80002),
        ("arbitrum-one".into(), 42161),
        ("arbitrum-nova".into(), 42170),
        ("arbitrum-sepolia".into(), 421614),
        ("zksync".into(), 324),
        ("zksync-sepolia".into(), 300),
        ("flow".into(), 747),
        ("flow-testnet".into(), 545),
        ("scroll".into(), 534352),
        ("scroll-sepolia".into(), 534351),
        ("mantle".into(), 5000),
        ("linea".into(), 59144),
        ("linea-sepolia".into(), 59141),
        ("bitkub-chain".into(), 96),
        ("bitkub-chain-testnet".into(), 25925),
        ("kinto".into(), 7887),
        ("celo".into(), 42220),
        ("zircuit1".into(), 48900),
        ("zircuit1-testnet".into(), 48899),
        ("gnosis".into(), 100),
        ("gnosis-chiado".into(), 10200),
        ("phenix".into(), 8008135),
    ])
});

static ETH_MAINNET_CHAIN_SPEC: Lazy<ChainSpec> =
    Lazy::new(|| load_chain_spec_from_file().expect("Failed to load chain configuration"));

pub static ETH_SEPOLIA_CHAIN_SPEC: Lazy<ChainSpec> = Lazy::new(|| {
    ChainSpec::new(
        Chain::sepolia().id(),
        [
            Fork::after_block(MERGE, 1735371),
            Fork::after_timestamp(SHANGHAI, 1677557088),
            Fork::after_timestamp(CANCUN, 1706655072),
        ],
    )
});

pub static BASE_CHAIN_SPEC: Lazy<ChainSpec> = Lazy::new(|| {
    ChainSpec::new(
        Chain::base_mainnet().id(),
        [
            Fork::after_block(MERGE, 1735371),
            Fork::after_timestamp(SHANGHAI, 1677557088),
            Fork::after_timestamp(CANCUN, 1706655072),
        ],
    )
});

pub static BASE_SEPOLIA_CHAIN_SPEC: Lazy<ChainSpec> = Lazy::new(|| {
    ChainSpec::new(
        Chain::base_sepolia().id(),
        [
            Fork::after_block(MERGE, 1735371),
            Fork::after_timestamp(SHANGHAI, 1677557088),
            Fork::after_timestamp(CANCUN, 1706655072),
        ],
    )
});

pub static OP_MAINNET_CHAIN_SPEC: Lazy<ChainSpec> = Lazy::new(|| {
    ChainSpec::new(
        Chain::optimism_mainnet().id(),
        [
            Fork::after_block(MERGE, 1735371),
            Fork::after_timestamp(SHANGHAI, 1677557088),
            Fork::after_timestamp(CANCUN, 1706655072),
        ],
    )
});

pub static OP_SEPOLIA_CHAIN_SPEC: Lazy<ChainSpec> = Lazy::new(|| {
    ChainSpec::new(
        Chain::optimism_sepolia().id(),
        [
            Fork::after_block(MERGE, 1735371),
            Fork::after_timestamp(SHANGHAI, 1677557088),
            Fork::after_timestamp(CANCUN, 1706655072),
        ],
    )
});

pub static POLYGON_CHAIN_SPEC: Lazy<ChainSpec> = Lazy::new(|| {
    ChainSpec::new(
        137,
        [
            Fork::after_block(MERGE, 1735371),
            Fork::after_timestamp(SHANGHAI, 1677557088),
            Fork::after_timestamp(CANCUN, 1706655072),
        ],
    )
});

pub static POLYGON_AMOY_CHAIN_SPEC: Lazy<ChainSpec> = Lazy::new(|| {
    ChainSpec::new(
        80002,
        [
            Fork::after_block(MERGE, 1735371),
            Fork::after_timestamp(SHANGHAI, 1677557088),
            Fork::after_timestamp(CANCUN, 1706655072),
        ],
    )
});

pub static ARBITRUM_NOVA_CHAIN_SPEC: Lazy<ChainSpec> = Lazy::new(|| {
    ChainSpec::new(
        42170,
        [
            Fork::after_block(MERGE, 1735371),
            Fork::after_timestamp(SHANGHAI, 1677557088),
            Fork::after_timestamp(CANCUN, 1706655072),
        ],
    )
});

pub static ARBITRUM_ONE_CHAIN_SPEC: Lazy<ChainSpec> = Lazy::new(|| {
    ChainSpec::new(
        42161,
        [
            Fork::after_block(MERGE, 1735371),
            Fork::after_timestamp(SHANGHAI, 1677557088),
            Fork::after_timestamp(CANCUN, 1706655072),
        ],
    )
});

pub static ARBITRUM_SEPOLIA_CHAIN_SPEC: Lazy<ChainSpec> = Lazy::new(|| {
    ChainSpec::new(
        421614,
        [
            Fork::after_block(MERGE, 1735371),
            Fork::after_timestamp(SHANGHAI, 1677557088),
            Fork::after_timestamp(CANCUN, 1706655072),
        ],
    )
});

pub static ZKSYNC_CHAIN_SPEC: Lazy<ChainSpec> = Lazy::new(|| {
    ChainSpec::new(
        324,
        [
            Fork::after_block(MERGE, 1735371),
            Fork::after_timestamp(SHANGHAI, 1677557088),
            Fork::after_timestamp(CANCUN, 1706655072),
        ],
    )
});

pub static ZKSYNC_SEPOLIA_CHAIN_SPEC: Lazy<ChainSpec> = Lazy::new(|| {
    ChainSpec::new(
        300,
        [
            Fork::after_block(MERGE, 1735371),
            Fork::after_timestamp(SHANGHAI, 1677557088),
            Fork::after_timestamp(CANCUN, 1706655072),
        ],
    )
});

pub static TESTING_CHAIN_SPEC: Lazy<ChainSpec> =
    Lazy::new(|| ChainSpec::new_single(TEST_CHAIN_ID, MERGE));

pub static FLOW_MAINNET_CHAIN_SPEC: Lazy<ChainSpec> =
    Lazy::new(|| ChainSpec::new_single(747, CANCUN));

pub static FLOW_TESTNET_CHAIN_SPEC: Lazy<ChainSpec> =
    Lazy::new(|| ChainSpec::new_single(545, CANCUN));

pub static SCROLL_MAINNET_CHAIN_SPEC: Lazy<ChainSpec> =
    Lazy::new(|| ChainSpec::new_single(534352, CANCUN));

pub static SCROLL_SEPOLIA_CHAIN_SPEC: Lazy<ChainSpec> =
    Lazy::new(|| ChainSpec::new_single(534351, CANCUN));

pub static MANTLE_MAINNET_CHAIN_SPEC: Lazy<ChainSpec> =
    Lazy::new(|| ChainSpec::new_single(5000, CANCUN));

pub static LINEA_MAINNET_CHAIN_SPEC: Lazy<ChainSpec> =
    Lazy::new(|| ChainSpec::new_single(59144, CANCUN));

pub static LINEA_SEPOLIA_CHAIN_SPEC: Lazy<ChainSpec> =
    Lazy::new(|| ChainSpec::new_single(59141, CANCUN));

pub static BITKUB_CHAIN_CHAIN_SPEC: Lazy<ChainSpec> =
    Lazy::new(|| ChainSpec::new_single(96, CANCUN));

pub static BITKUB_CHAIN_TESTNET_CHAIN_SPEC: Lazy<ChainSpec> =
    Lazy::new(|| ChainSpec::new_single(25925, CANCUN));

pub static KINTO_CHAIN_SPEC: Lazy<ChainSpec> = Lazy::new(|| ChainSpec::new_single(7887, CANCUN));

pub static CELO_CHAIN_SPEC: Lazy<ChainSpec> = Lazy::new(|| ChainSpec::new_single(42220, CANCUN));

pub static ZIRCUIT1_MAINNET_CHAIN_SPEC: Lazy<ChainSpec> =
    Lazy::new(|| ChainSpec::new_single(48900, CANCUN));

pub static ZIRCUIT1_TESTNET_CHAIN_SPEC: Lazy<ChainSpec> =
    Lazy::new(|| ChainSpec::new_single(48899, CANCUN));

pub static GNOSIS_MAINNET_CHAIN_SPEC: Lazy<ChainSpec> =
    Lazy::new(|| ChainSpec::new_single(100, CANCUN));

pub static GNOSIS_CHIADO_CHAIN_SPEC: Lazy<ChainSpec> =
    Lazy::new(|| ChainSpec::new_single(10200, CANCUN));

pub static PHENIX_CHAIN_SPEC: Lazy<ChainSpec> =
    Lazy::new(|| ChainSpec::new_single(8008135, CANCUN));

fn load_chain_spec_from_file() -> Result<ChainSpec, Error> {
    let chain_spec = include_str!("../chain_spec.toml");
    toml::from_str(chain_spec).map_err(Error::from)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn read_chain_spec_from_file() {
        let chain_spec = load_chain_spec_from_file().expect("Failed to load chain spec from file");
        dbg!(chain_spec);
    }
}

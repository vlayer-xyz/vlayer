//! Handling different blockchain specifications.
use std::collections::HashMap;

use alloy_chains::Chain;
use alloy_primitives::ChainId;
use once_cell::sync::Lazy;
use serde::Deserialize;
use toml::from_str;

use crate::spec::ChainSpec;

// Some unique chain ids for testing
pub const TEST_CHAIN_ID: ChainId = 31_337;

// https://etherscan.io/block/15537394
pub const MAINNET_MERGE_BLOCK_NUMBER: u64 = 15537394;
pub const MAINNET_MERGE_BLOCK_TIMESTAMP: u64 = 1663224179;

pub static CHAIN_ID_TO_CHAIN_SPEC: Lazy<HashMap<ChainId, ChainSpec>> = Lazy::new(load_chain_specs);

#[derive(Debug, Deserialize)]
struct ChainSpecs {
    pub chains: Vec<ChainSpec>,
}

fn load_chain_specs() -> HashMap<ChainId, ChainSpec> {
    // include_str! loads chain_specs in compilation time
    let chain_specs: ChainSpecs =
        from_str(include_str!("../chain_specs.toml")).expect("failed to parse chain specs");
    let chain_specs_len = chain_specs.chains.len();
    let chain_id_to_chain_spec: HashMap<ChainId, ChainSpec> = chain_specs
        .chains
        .into_iter()
        .map(|chain| (*chain, chain))
        .collect();

    assert!(chain_specs_len == chain_id_to_chain_spec.len(), "duplicated chain specs",);

    chain_id_to_chain_spec
}

pub static CHAIN_NAME_TO_ID: Lazy<HashMap<String, ChainId>> = Lazy::new(|| {
    [
        ("mainnet", Chain::mainnet().id()),
        ("sepolia", Chain::sepolia().id()),
        ("base", Chain::base_mainnet().id()),
        ("base-sepolia", Chain::base_sepolia().id()),
        ("optimism", Chain::optimism_mainnet().id()),
        ("optimism-sepolia", Chain::optimism_sepolia().id()),
        ("polygon", 137),
        ("polygon-amoy", 80002),
        ("arbitrum-one", 42161),
        ("arbitrum-nova", 42170),
        ("arbitrum-sepolia", 421614),
        ("zksync", 324),
        ("zksync-sepolia", 300),
        ("linea", 59144),
        ("linea-sepolia", 59141),
    ]
    .iter()
    .map(|(k, v)| (k.to_string(), *v))
    .collect()
});

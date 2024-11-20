//! Handling different blockchain specifications.
use std::collections::HashMap;

use alloy_primitives::ChainId;
use once_cell::sync::Lazy;
use serde::Deserialize;
use toml::from_str;

use crate::spec::ChainSpec;

// Some unique chain ids for testing
pub const TEST_CHAIN_ID: ChainId = 31_337;

// https://etherscan.io/block/15537394
pub const MAINNET_MERGE_BLOCK_NUMBER: u64 = 15_537_394;
pub const MAINNET_MERGE_BLOCK_TIMESTAMP: u64 = 1_663_224_179;

pub static CHAIN_ID_TO_CHAIN_SPEC: Lazy<HashMap<ChainId, ChainSpec>> =
    Lazy::new(load_chain_id_to_chain_spec);

pub static CHAIN_NAME_TO_CHAIN_ID: Lazy<HashMap<String, ChainId>> =
    Lazy::new(load_chain_name_to_chain_id);

fn load_chain_id_to_chain_spec() -> HashMap<ChainId, ChainSpec> {
    let mut chain_id_to_chain_spec = HashMap::with_capacity(CHAIN_SPECS.chains.len());

    for chain in &CHAIN_SPECS.chains {
        if chain_id_to_chain_spec.contains_key(&chain.chain_id) {
            panic!("duplicated chain spec for ID {}", chain.chain_id);
        }
        chain_id_to_chain_spec.insert(chain.chain_id, chain.clone());
    }

    chain_id_to_chain_spec
}

fn load_chain_name_to_chain_id() -> HashMap<String, ChainId> {
    let mut chain_name_to_id = HashMap::with_capacity(CHAIN_SPECS.chains.len());

    for chain in &CHAIN_SPECS.chains {
        if chain_name_to_id.contains_key(&chain.name) {
            panic!("duplicated chain spec for name {}", chain.name);
        }
        chain_name_to_id.insert(chain.name.clone(), chain.chain_id);
    }

    chain_name_to_id
}

#[derive(Debug, Deserialize)]
struct ChainSpecs {
    pub chains: Vec<ChainSpec>,
}

static CHAIN_SPECS: Lazy<ChainSpecs> = Lazy::new(|| {
    // include_str! loads chain_specs in compilation time
    from_str(include_str!("../chain_specs.toml")).expect("failed to parse chain specs")
});

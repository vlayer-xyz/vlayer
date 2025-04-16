//! Handling different blockchain specifications.
use std::collections::{HashMap, HashSet};

use alloy_primitives::ChainId;
use lazy_static::lazy_static;
use serde::Deserialize;
use toml::from_str;

use crate::spec::ChainSpec;

// Some unique chain ids for testing
pub const TEST_CHAIN_ID: ChainId = 30_1337;

// https://etherscan.io/block/15537394
pub const MAINNET_MERGE_BLOCK_NUMBER: u64 = 15_537_394;
pub const MAINNET_MERGE_BLOCK_TIMESTAMP: u64 = 1_663_224_179;

lazy_static! {
    pub static ref CHAIN_ID_TO_CHAIN_SPEC: HashMap<ChainId, ChainSpec> =
        chain_id_to_spec_map();

    pub static ref CHAIN_NAME_TO_CHAIN_ID: HashMap<String, ChainId> =
        chain_name_to_id_map();

    static ref CHAIN_SPECS: ChainSpecs = {
        // `include_str!` includes the file contents at compile time
        #[expect(clippy::expect_used)]
        let specs: ChainSpecs = from_str(include_str!("../chain_specs.toml"))
            .expect("failed to parse chain specs");
        specs.assert_no_duplicates();
        specs
    };
}

#[derive(Debug, Deserialize)]
struct ChainSpecs {
    pub chains: Vec<ChainSpec>,
}

impl ChainSpecs {
    #[allow(clippy::panic)]
    pub fn assert_no_duplicates(&self) {
        let mut chain_ids = HashSet::new();
        let mut chain_names = HashSet::new();

        for chain in &self.chains {
            if !chain_ids.insert(chain.id()) {
                panic!("duplicate chain id: {}", chain.id());
            }

            if !chain_names.insert(chain.name()) {
                panic!("duplicate chain name: {}", chain.name());
            }
        }
    }
}

fn chain_id_to_spec_map() -> HashMap<ChainId, ChainSpec> {
    CHAIN_SPECS
        .chains
        .iter()
        .map(|chain| (chain.id(), chain.clone()))
        .collect()
}

fn chain_name_to_id_map() -> HashMap<String, ChainId> {
    CHAIN_SPECS
        .chains
        .iter()
        .map(|chain| (chain.name().to_string(), chain.id()))
        .collect()
}

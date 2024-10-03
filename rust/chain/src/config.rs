//! Handling different blockchain specifications.
use std::collections::{BTreeMap, HashMap};

use alloy_chains::Chain;
use alloy_primitives::ChainId;
use once_cell::sync::Lazy;
use revm::primitives::SpecId;

use crate::{eip1559::Eip1559Constants, fork::ForkCondition, spec::ChainSpec};

// Some unique chain ids for testing
pub const TEST_CHAIN_ID_1: ChainId = 100001;

pub const MAINNET_MERGE_BLOCK_NUMBER: u64 = 15537394;

pub static CHAIN_MAP: Lazy<HashMap<ChainId, &'static Lazy<ChainSpec>>> = Lazy::new(|| {
    HashMap::from([
        (Chain::mainnet().id(), &ETH_MAINNET_CHAIN_SPEC),
        (Chain::sepolia().id(), &ETH_SEPOLIA_CHAIN_SPEC),
        (TEST_CHAIN_ID_1, &TESTING_CHAIN_SPEC),
        (Chain::base_mainnet().id(), &TESTING_CHAIN_SPEC),
        (Chain::optimism_mainnet().id(), &TESTING_CHAIN_SPEC),
    ])
});

pub static CHAIN_NAMES: Lazy<HashMap<String, ChainId>> = Lazy::new(|| {
    HashMap::from([
        ("mainnet".into(), Chain::mainnet().id()),
        ("sepolia".into(), Chain::sepolia().id()),
    ])
});

pub static ETH_MAINNET_CHAIN_SPEC: Lazy<ChainSpec> = Lazy::new(|| {
    ChainSpec::new(
        Chain::mainnet().id(),
        SpecId::CANCUN,
        BTreeMap::from([
            (SpecId::MERGE, ForkCondition::Block(MAINNET_MERGE_BLOCK_NUMBER)),
            (SpecId::SHANGHAI, ForkCondition::Timestamp(1681338455)),
            (SpecId::CANCUN, ForkCondition::Timestamp(1710338135)),
        ]),
        BTreeMap::from([(SpecId::LONDON, Eip1559Constants::default())]),
    )
});

pub static ETH_SEPOLIA_CHAIN_SPEC: Lazy<ChainSpec> = Lazy::new(|| {
    ChainSpec::new(
        Chain::sepolia().id(),
        SpecId::CANCUN,
        BTreeMap::from([
            (SpecId::MERGE, ForkCondition::Block(1735371)),
            (SpecId::SHANGHAI, ForkCondition::Timestamp(1677557088)),
            (SpecId::CANCUN, ForkCondition::Timestamp(1706655072)),
        ]),
        BTreeMap::from([(SpecId::LONDON, Eip1559Constants::default())]),
    )
});

pub static TESTING_CHAIN_SPEC: Lazy<ChainSpec> = Lazy::new(|| {
    ChainSpec::new(
        TEST_CHAIN_ID_1,
        SpecId::CANCUN,
        BTreeMap::from([
            (SpecId::MERGE, ForkCondition::Block(0)),
            (SpecId::SHANGHAI, ForkCondition::Block(0)),
            (SpecId::CANCUN, ForkCondition::Block(0)),
        ]),
        BTreeMap::from([(SpecId::LONDON, Eip1559Constants::default())]),
    )
});

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn spec_id() {
        assert_eq!(ETH_MAINNET_CHAIN_SPEC.spec_id(15537393, 0), None);
        assert_eq!(ETH_MAINNET_CHAIN_SPEC.spec_id(15537394, 0), Some(SpecId::MERGE));
        assert_eq!(ETH_MAINNET_CHAIN_SPEC.spec_id(17034869, 0), Some(SpecId::MERGE));
        assert_eq!(ETH_MAINNET_CHAIN_SPEC.spec_id(0, 1681338455), Some(SpecId::SHANGHAI));
    }

    #[test]
    fn gas_constants() {
        assert_eq!(ETH_MAINNET_CHAIN_SPEC.gas_constants(SpecId::BERLIN), None);
        assert_eq!(
            ETH_MAINNET_CHAIN_SPEC.gas_constants(SpecId::MERGE),
            Some(&Eip1559Constants::default())
        );
        assert_eq!(
            ETH_MAINNET_CHAIN_SPEC.gas_constants(SpecId::SHANGHAI),
            Some(&Eip1559Constants::default())
        );
    }
}

//! Handling different blockchain specifications.
use std::collections::BTreeMap;

use alloy_primitives::{address, Address};
use once_cell::sync::Lazy;
use revm::primitives::SpecId;
use std::collections::HashMap;

use crate::chain::{eip1559::Eip1559Constants, fork::ForkCondition, spec::ChainSpec};

pub const MAINNET_ID: u64 = 1;
pub const SEPOLIA_ID: u64 = 11155111;
pub const MAINNET_MERGE_BLOCK_NUMBER: u64 = 15537394;

pub const DEFAULT_CALLER: Address = address!("1111111111111111111111111111111111111111");

pub static CHAIN_MAP: Lazy<HashMap<u64, &'static Lazy<ChainSpec>>> = Lazy::new(|| {
    HashMap::from([
        (MAINNET_ID, &ETH_MAINNET_CHAIN_SPEC),
        (SEPOLIA_ID, &ETH_SEPOLIA_CHAIN_SPEC),
    ])
});

pub static ETH_MAINNET_CHAIN_SPEC: Lazy<ChainSpec> = Lazy::new(|| {
    ChainSpec::new(
        MAINNET_ID,
        SpecId::CANCUN,
        BTreeMap::from([
            (
                SpecId::MERGE,
                ForkCondition::Block(MAINNET_MERGE_BLOCK_NUMBER),
            ),
            (SpecId::SHANGHAI, ForkCondition::Timestamp(1681338455)),
            (SpecId::CANCUN, ForkCondition::Timestamp(1710338135)),
        ]),
        BTreeMap::from([(SpecId::LONDON, Eip1559Constants::default())]),
    )
});

pub static ETH_SEPOLIA_CHAIN_SPEC: Lazy<ChainSpec> = Lazy::new(|| {
    ChainSpec::new(
        SEPOLIA_ID,
        SpecId::CANCUN,
        BTreeMap::from([
            (SpecId::MERGE, ForkCondition::Block(1735371)),
            (SpecId::SHANGHAI, ForkCondition::Timestamp(1677557088)),
            (SpecId::CANCUN, ForkCondition::Timestamp(1706655072)),
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
        assert_eq!(
            ETH_MAINNET_CHAIN_SPEC.spec_id(15537394, 0),
            Some(SpecId::MERGE)
        );
        assert_eq!(
            ETH_MAINNET_CHAIN_SPEC.spec_id(17034869, 0),
            Some(SpecId::MERGE)
        );
        assert_eq!(
            ETH_MAINNET_CHAIN_SPEC.spec_id(0, 1681338455),
            Some(SpecId::SHANGHAI)
        );
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

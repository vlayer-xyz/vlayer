use std::collections::HashMap;

use alloy_primitives::{Address, U256, address};
use lazy_static::lazy_static;

pub const DEFAULT_CALLER: Address = address!("1111111111111111111111111111111111111111");
// Has the same meaning as coinbase in Ethereum.
pub const OPTIMISM_SEQUENCER_VAULT: Address = address!("4200000000000000000000000000000000000011");
// Simple contract that returns information about the latest L1 block
pub const L1_BLOCK: Address = address!("4200000000000000000000000000000000000015");
// Contract collecting base fees
pub const BASE_FEE_VAULT: Address = address!("4200000000000000000000000000000000000019");
// Contract collecting L1 fees
pub const L1_FEE_VAULT: Address = address!("420000000000000000000000000000000000001a");

pub static EMPTY_ACCOUNTS: &[Address] =
    &[DEFAULT_CALLER, OPTIMISM_SEQUENCER_VAULT, L1_BLOCK, BASE_FEE_VAULT, L1_FEE_VAULT];

pub type Storage = HashMap<U256, U256>;

lazy_static! {
    static ref L1_BLOCK_STORAGE: Storage = {
        HashMap::from([
            // number
            (U256::from(1), U256::from(0)),
            // basefee
            (U256::from(3), U256::from(0)),
            // sequenceNumber
            (U256::from(5), U256::from(0)),
            // blobBaseFeeScalar
            (U256::from(6), U256::from(0)),
            // baseFeeScalar
            (U256::from(7), U256::from(0)),
        ])
    };
    pub static ref ACCOUNT_TO_STORAGE: HashMap<Address, Storage> = {
        HashMap::from([
            (L1_BLOCK, L1_BLOCK_STORAGE.clone()),
        ])
    };
}

use alloy_primitives::{address, Address};

pub const DEFAULT_CALLER: Address = address!("1111111111111111111111111111111111111111");
// Has the same meaning as coinbase in Ethereum.
pub const OPTIMISM_SEQUENCER_VAULT: Address = address!("4200000000000000000000000000000000000011");
// Simple contract that returns information about the latest L1 block
pub const L1_BLOCK: Address = address!("4200000000000000000000000000000000000015");
// Contract collecting base fees
pub const BASE_FEE_VAULT: Address = address!("4200000000000000000000000000000000000019");
// Contract collecting L1 fees
pub const L1_FEE_VAULT: Address = address!("420000000000000000000000000000000000001a");

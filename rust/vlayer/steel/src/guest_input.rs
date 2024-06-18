use alloy_primitives::Address;
use serde::{Deserialize, Serialize};

use crate::ethereum::EthEvmInput;

#[derive(Deserialize, Serialize, Debug)]
pub struct GuestInput {
    pub evm_input: EthEvmInput,
    pub call: Call,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct Call {
    pub caller: Address,
    pub to: Address,
    pub data: Vec<u8>,
}

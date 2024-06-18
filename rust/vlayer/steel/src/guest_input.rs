use alloy_primitives::Address;
use serde::{Deserialize, Serialize};

use crate::ethereum::EthEvmInput;

#[derive(Deserialize, Serialize, Debug)]
pub struct GuestInput {
    pub evm_input: EthEvmInput,
    pub call_data: Vec<u8>,
    pub caller: Address,
    pub to: Address,
}

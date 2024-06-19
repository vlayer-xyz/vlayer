use alloy_primitives::Address;
use risc0_zkvm::Journal;
use serde::{Deserialize, Serialize};

use crate::ethereum::EthEvmInput;
use crate::SolCommitment;

#[derive(Deserialize, Serialize, Debug)]
pub struct Input {
    pub evm_input: EthEvmInput,
    pub call: Call,
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct Call {
    pub caller: Address,
    pub to: Address,
    pub data: Vec<u8>,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct Output {
    pub block_commitment: SolCommitment,
    pub evm_call_result: Vec<u8>,
}

impl From<Journal> for Output {
    fn from(value: Journal) -> Self {
        value
            .decode()
            .expect("Could not decode Journal into GuestOutput")
    }
}

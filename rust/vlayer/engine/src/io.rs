use alloy_primitives::Address;
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

#[derive(Debug)]
pub struct Output {
    pub execution_commitment: SolCommitment,
    pub evm_call_result: Vec<u8>,
}

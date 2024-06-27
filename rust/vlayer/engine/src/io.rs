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

impl Default for Call {
    fn default() -> Self {
        Self {
            caller: Address::ZERO,
            to: Address::ZERO,
            data: vec![],
        }
    }
}

pub struct Output {
    pub execution_commitment: SolCommitment,
    pub evm_call_result: Vec<u8>,
}

pub struct GuestOutput {
    pub execution_commitment: SolCommitment,
    pub evm_call_result: Vec<u8>,
}

pub struct HostOutput {
    pub raw_abi: Vec<u8>,
    pub guest_output: GuestOutput,
}

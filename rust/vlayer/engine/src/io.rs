use alloy_primitives::Address;
use alloy_sol_types::SolValue;
use serde::{Deserialize, Serialize};

use crate::ethereum::EthEvmInput;
use crate::ExecutionCommitment;

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

pub struct GuestOutput {
    pub execution_commitment: ExecutionCommitment,
    pub evm_call_result: Vec<u8>,
}

impl GuestOutput {
    pub fn from_outputs(host_output: &[u8], guest_output: &[u8]) -> Self {
        let execution_commitment_len = guest_output.len() - host_output.len();

        let (execution_commitment_abi_encoded, evm_call_result_abi_encoded) =
            guest_output.split_at(execution_commitment_len);

        assert_eq!(host_output, evm_call_result_abi_encoded);

        let execution_commitment =
            ExecutionCommitment::abi_decode(execution_commitment_abi_encoded, true)
                .expect("Cannot decode execution commitment");
        Self {
            execution_commitment,
            evm_call_result: evm_call_result_abi_encoded.to_vec(),
        }
    }
}

pub struct HostOutput {
    pub raw_abi: Vec<u8>,
    pub guest_output: GuestOutput,
}

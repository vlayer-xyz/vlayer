use alloy_primitives::Address;
use alloy_sol_types::SolValue;
use serde::{Deserialize, Serialize};
use thiserror::Error;

use crate::ethereum::MultiEthEvmInput;
use crate::evm::env::ExecutionLocation;
use crate::ExecutionCommitment;

#[derive(Deserialize, Serialize, Debug)]
pub struct Input {
    pub evm_input: MultiEthEvmInput,
    pub call: Call,
    pub start_execution_location: ExecutionLocation,
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

#[derive(Error, Debug)]
pub enum GuestOutputError {
    #[error("Cannot decode execution commitment: {0}")]
    CannonDecodeExecutionCommitment(String),
}
pub struct GuestOutput {
    pub execution_commitment: ExecutionCommitment,
    pub evm_call_result: Vec<u8>,
}

impl GuestOutput {
    pub fn from_outputs(host_output: &[u8], guest_output: &[u8]) -> Result<Self, GuestOutputError> {
        let execution_commitment_len = guest_output.len() - host_output.len();

        let (execution_commitment_abi_encoded, evm_call_result_abi_encoded) =
            guest_output.split_at(execution_commitment_len);

        let execution_commitment =
            ExecutionCommitment::abi_decode(execution_commitment_abi_encoded, true).map_err(
                |err| GuestOutputError::CannonDecodeExecutionCommitment(err.to_string()),
            )?;

        Ok(Self {
            execution_commitment,
            evm_call_result: evm_call_result_abi_encoded.to_vec(),
        })
    }
}

pub struct HostOutput {
    pub raw_abi: Vec<u8>,
    pub guest_output: GuestOutput,
}

use alloy_primitives::{Address, TxKind};
use alloy_sol_types::SolValue;
use revm::primitives::TxEnv;
use serde::{Deserialize, Serialize};
use thiserror::Error;

use crate::evm::env::location::ExecutionLocation;
use crate::evm::input::MultiEvmInput;
use crate::ExecutionCommitment;

#[derive(Deserialize, Serialize, Debug)]
pub struct Input {
    pub multi_evm_input: MultiEvmInput,
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

impl From<Call> for TxEnv {
    fn from(call: Call) -> Self {
        Self {
            caller: call.caller,
            transact_to: match call.to {
                Address::ZERO => TxKind::Create,
                to => TxKind::Call(to),
            },
            data: call.data.into(),
            ..Default::default()
        }
    }
}

pub type CallSelector = [u8; 4];

impl Call {
    pub fn selector(&self) -> CallSelector {
        self.data[0..4]
            .try_into()
            .expect("cannot extract function selector from call data")
    }
}

#[derive(Error, Debug)]
pub enum GuestOutputError {
    #[error("Cannot decode execution commitment: {0}")]
    CannonDecodeExecutionCommitment(String),
}

#[derive(Debug)]
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

#[derive(Debug)]
pub struct HostOutput {
    pub raw_abi: Vec<u8>,
    pub seal: Vec<u8>,
    pub guest_output: GuestOutput,
}

#[cfg(test)]
mod from_call_to_tx_env {
    use alloy_primitives::Bytes;

    use super::*;

    #[test]
    fn data() {
        let tx_env: TxEnv = Call {
            data: vec![4, 2],
            ..Default::default()
        }
        .into();
        assert_eq!(tx_env.data, Bytes::from(vec![4, 2]));
    }

    #[test]
    fn creation_call() {
        let tx_env: TxEnv = Call {
            to: Address::ZERO,
            ..Default::default()
        }
        .into();
        assert_eq!(tx_env.transact_to, TxKind::Create);
    }

    #[test]
    fn non_creation_call() {
        let non_zero_address = Address::from_slice(&[1; 20]);
        let tx_env: TxEnv = Call {
            to: non_zero_address,
            ..Default::default()
        }
        .into();
        assert_eq!(tx_env.transact_to, TxKind::Call(non_zero_address));
    }
}

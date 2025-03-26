use std::time::Duration;

use alloy_primitives::{Address, Bytes as RlpBytes, FixedBytes, TxKind};
use alloy_rlp::RlpEncodable;
use alloy_sol_types::{SolCall, SolValue};
use bytes::Bytes;
use call_common::ExecutionLocation;
use chain_client::ChainProofCache;
use derive_new::new;
use optimism::client::factory::cached::OpOutputCache;
use revm::{
    interpreter::CallInputs,
    primitives::{OptimismFields, TxEnv},
};
use risc0_zkvm::sha::Digest;
use serde::{Deserialize, Serialize};
use thiserror::Error;

use crate::{CallAssumptions, config::DEFAULT_CALLER, evm::input::MultiEvmInput};

#[derive(Deserialize, Serialize, Debug, Default, Clone)]
pub struct Input {
    pub multi_evm_input: MultiEvmInput,
    pub start_execution_location: ExecutionLocation,
    pub chain_proofs: ChainProofCache,
    pub call: Call,
    pub op_output_cache: OpOutputCache,
    pub is_vlayer_test: bool,
}

#[derive(Deserialize, Serialize, Debug, Clone, RlpEncodable)]
pub struct Call {
    pub to: Address,
    pub data: Vec<u8>,
    pub gas_limit: u64,
}

impl Call {
    pub fn new(to: Address, call: &impl SolCall, gas_limit: u64) -> Self {
        Self {
            to,
            data: call.abi_encode(),
            gas_limit,
        }
    }
}

impl Default for Call {
    fn default() -> Self {
        Self {
            to: Address::ZERO,
            data: vec![],
            gas_limit: 30_000_000,
        }
    }
}

impl From<Call> for TxEnv {
    fn from(call: Call) -> Self {
        Self {
            caller: DEFAULT_CALLER,
            transact_to: match call.to {
                Address::ZERO => TxKind::Create,
                to => TxKind::Call(to),
            },
            gas_limit: call.gas_limit,
            data: call.data.into(),
            optimism: initialize_optimism_fields(),
            ..Default::default()
        }
    }
}

fn initialize_optimism_fields() -> OptimismFields {
    OptimismFields {
        enveloped_tx: Some(RlpBytes::default()),
        ..Default::default()
    }
}

impl From<&CallInputs> for Call {
    fn from(inputs: &CallInputs) -> Self {
        Self {
            to: inputs.bytecode_address,
            data: inputs.input.clone().into(),
            gas_limit: inputs.gas_limit,
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
    #[error("Cannot decode call assumptions: {0}")]
    CannotDecodeCallAssumptions(String),
}

#[derive(Clone, Debug, new)]
pub struct GuestOutput {
    pub call_assumptions: CallAssumptions,
    pub evm_call_result: Vec<u8>,
}

impl GuestOutput {
    pub fn from_outputs(host_output: &[u8], guest_output: &[u8]) -> Result<Self, GuestOutputError> {
        let call_assumptions_len = guest_output.len() - host_output.len();

        let (call_assumptions_abi_encoded, evm_call_result_abi_encoded) =
            guest_output.split_at(call_assumptions_len);

        let call_assumptions = CallAssumptions::abi_decode(call_assumptions_abi_encoded, true)
            .map_err(|err| GuestOutputError::CannotDecodeCallAssumptions(err.to_string()))?;

        Ok(Self {
            call_assumptions,
            evm_call_result: evm_call_result_abi_encoded.to_vec(),
        })
    }
}

#[derive(Clone, Debug)]
pub struct HostOutput {
    pub raw_abi: Bytes,
    pub seal: Bytes,
    pub guest_output: GuestOutput,
    pub proof_len: usize,
    pub call_guest_id: CallGuestId,
    pub cycles_used: u64,
    pub elapsed_time: Duration,
}

#[derive(Debug, Clone)]
pub struct CallGuestId(Digest);

impl From<Digest> for CallGuestId {
    fn from(value: Digest) -> Self {
        Self(value)
    }
}

impl From<CallGuestId> for FixedBytes<32> {
    fn from(value: CallGuestId) -> Self {
        let mut bytes = [0_u8; 32];
        bytes.copy_from_slice(value.0.as_bytes());

        Self::new(bytes)
    }
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

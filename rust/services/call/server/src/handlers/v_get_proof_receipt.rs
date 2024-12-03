use alloy_primitives::{hex::ToHexExt, U256};
use alloy_sol_types::SolValue;
use call_engine::{HostOutput, Proof, Seal};
use call_host::Error as HostError;
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};
use tracing::info;

use super::SharedState;
use crate::{error::AppError, v_call::CallHash};

#[derive(Deserialize, Serialize, Debug)]
pub struct Params {
    hash: CallHash,
}

pub async fn v_get_proof_receipt(
    state: SharedState,
    params: Params,
) -> Result<CallResult, AppError> {
    info!("v_get_proof_receipt => {params:#?}");
    match state.lock().hashes.get(&params.hash) {
        Some(host_output) => Ok(CallResult::try_new(host_output.clone())?),
        None => Err(AppError::HashNotFound(params.hash.to_string())),
    }
}

#[allow(clippy::struct_field_names)]
pub struct CallResult {
    pub proof: Proof,
    pub evm_call_result: Vec<u8>,
}

impl CallResult {
    pub fn try_new(host_output: HostOutput) -> Result<Self, HostError> {
        let HostOutput {
            guest_output,
            seal,
            proof_len,
            call_guest_id,
            ..
        } = host_output;

        let proof = Proof {
            length: U256::from(proof_len),
            seal: decode_seal(&seal)?,
            callGuestId: call_guest_id.into(),
            // Intentionally set to 0. These fields will be updated with the correct values by the prover script, based on the verifier ABI.
            callAssumptions: guest_output.call_assumptions,
        };
        Ok(Self {
            proof,
            evm_call_result: guest_output.evm_call_result,
        })
    }

    pub fn to_json(&self) -> Value {
        json!({
            "evm_call_result": self.evm_call_result.encode_hex_with_prefix(),
            "proof": {
                "seal": {
                    "verifierSelector": self.proof.seal.verifierSelector,
                    "seal": self.proof.seal.seal,
                    "mode": Into::<u8>::into(self.proof.seal.mode),
                },
                "callGuestId": self.proof.callGuestId.encode_hex_with_prefix(),
                "length": u256_to_number(self.proof.length),
                "callAssumptions": {
                    "functionSelector": self.proof.callAssumptions.functionSelector,
                    "proverContractAddress": self.proof.callAssumptions.proverContractAddress,
                    "settleBlockNumber": u256_to_number(self.proof.callAssumptions.settleBlockNumber),
                    "settleBlockHash": self.proof.callAssumptions.settleBlockHash,
                }
            },
        })
    }
}

fn decode_seal(seal: &[u8]) -> Result<Seal, seal::Error> {
    Ok(Seal::abi_decode(seal, true)?)
}

fn u256_to_number(value: U256) -> u64 {
    u64::try_from(value).expect("Expected value to fit into u64")
}

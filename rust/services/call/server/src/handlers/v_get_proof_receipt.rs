use alloy_primitives::{hex::ToHexExt, U256};
use alloy_sol_types::SolValue;
use call_engine::{HostOutput, Proof, Seal};
use call_host::Error as HostError;
use serde::{Deserialize, Serialize, Serializer};
use tracing::info;

use super::SharedState;
use crate::{error::AppError, ser::ProofDTO, v_call::CallHash};

#[derive(Deserialize, Serialize, Debug)]
pub struct Params {
    hash: CallHash,
}

pub async fn v_get_proof_receipt(
    state: SharedState,
    params: Params,
) -> Result<CallResult, AppError> {
    info!("v_get_proof_receipt => {params:#?}");
    match state.read().get(&params.hash) {
        Some(host_output) => Ok(CallResult::try_new(host_output.clone())?),
        None => Err(AppError::HashNotFound(params.hash.to_string())),
    }
}

#[derive(Serialize)]
#[allow(clippy::struct_field_names)]
pub struct CallResult {
    #[serde(with = "ProofDTO")]
    pub proof: Proof,
    #[serde(serialize_with = "ser_evm_call_result")]
    pub evm_call_result: Vec<u8>,
}

fn ser_evm_call_result<S>(evm_call_result: &[u8], state: S) -> Result<S::Ok, S::Error>
where
    S: Serializer,
{
    state.serialize_str(&evm_call_result.encode_hex_with_prefix())
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
}

fn decode_seal(seal: &[u8]) -> Result<Seal, seal::Error> {
    Ok(Seal::abi_decode(seal, true)?)
}

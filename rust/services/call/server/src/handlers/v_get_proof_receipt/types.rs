use std::convert::TryFrom;

use alloy_primitives::{hex::ToHexExt, U256};
use alloy_sol_types::SolValue;
use call_engine::{HostOutput, Proof, Seal};
use call_host::Error as HostError;
use derive_new::new;
use serde::{Deserialize, Serialize, Serializer};

use crate::{error::AppError, ser::ProofDTO};

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "snake_case")]
pub enum Status {
    Pending,
    Done,
}

#[derive(Serialize)]
pub struct RawData {
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

impl TryFrom<HostOutput> for RawData {
    type Error = HostError;

    fn try_from(value: HostOutput) -> Result<Self, Self::Error> {
        let HostOutput {
            guest_output,
            seal,
            proof_len,
            call_guest_id,
            ..
        } = value;

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

#[derive(new, Serialize)]
pub struct CallResult {
    pub status: Status,
    pub data: Option<RawData>,
}

impl CallResult {
    pub fn from_maybe_output(output: Option<HostOutput>) -> Result<Self, AppError> {
        match output {
            None => Ok(Self::new(Status::Pending, None)),
            Some(output) => Ok(Self::new(Status::Done, Some(output.try_into()?))),
        }
    }
}

fn decode_seal(seal: &[u8]) -> Result<Seal, seal::Error> {
    Ok(Seal::abi_decode(seal, true)?)
}

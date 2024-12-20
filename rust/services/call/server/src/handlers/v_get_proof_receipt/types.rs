use std::convert::TryFrom;

use alloy_primitives::{hex::ToHexExt, U256};
use alloy_sol_types::SolValue;
use call_engine::{HostOutput, Proof, Seal};
use call_host::Error as HostError;
use derive_new::new;
use jsonrpsee::types::error::ErrorObjectOwned;
use serde::{Deserialize, Serialize, Serializer};

use crate::{error::AppError, handlers::ProofStatus, ser::ProofDTO};

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum Status {
    Queued,
    WaitingForChainProof,
    Preflight,
    Proving,
    Ready,
}

impl Default for Status {
    fn default() -> Self {
        Self::Queued
    }
}

impl From<&ProofStatus> for Status {
    fn from(value: &ProofStatus) -> Self {
        match value {
            ProofStatus::Queued => Self::Queued,
            ProofStatus::WaitingForChainProof => Self::WaitingForChainProof,
            ProofStatus::Preflight => Self::Preflight,
            ProofStatus::Proving => Self::Proving,
            ProofStatus::Ready(..) => Self::Ready,
        }
    }
}

#[derive(Serialize, Clone)]
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

#[derive(new, Clone, Serialize, Default)]
pub struct CallResult {
    pub status: Status,
    pub data: Option<RawData>,
}

impl TryFrom<&ProofStatus> for CallResult {
    type Error = ErrorObjectOwned;

    fn try_from(value: &ProofStatus) -> Result<Self, Self::Error> {
        let status: Status = value.into();
        let data: Option<RawData> = match value {
            ProofStatus::Ready(Ok(output)) => {
                Some(output.clone().try_into().map_err(AppError::from)?)
            }
            ProofStatus::Ready(Err(err)) => return Err(err.into()),
            _ => None,
        };
        Ok(Self { status, data })
    }
}

fn decode_seal(seal: &[u8]) -> Result<Seal, seal::Error> {
    Ok(Seal::abi_decode(seal, true)?)
}

use alloy_primitives::{hex::ToHexExt, U256};
use alloy_sol_types::SolValue;
use call_engine::CallGuestId;
use call_engine::{HostOutput, Proof, Seal};
use call_host::{Error as HostError, Host, PreflightResult, Prover};
use serde::{Serialize, Serializer};
use tracing::info;

use crate::{
    gas_meter::{Client as GasMeterClient, ComputationStage, Error as GasMeterError},
    handlers::{ProofStatus, SharedProofs},
    metrics::{self, Error as MetricsError, Metrics},
    ser::ProofDTO,
    v_call::CallHash,
};

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("Host error: {0}")]
    Host(#[from] HostError),
    #[error("Gas meter error: {0}")]
    GasMeter(#[from] GasMeterError),
    #[error("Metrics error: {0}")]
    Metrics(#[from] MetricsError),
    #[error("Seal error: {0}")]
    Seal(#[from] seal::Error),
}

pub async fn await_proving(
    prover: &Prover,
    state: &SharedProofs,
    call_hash: CallHash,
    call_guest_id: CallGuestId,
    preflight_result: PreflightResult,
    gas_meter_client: &impl GasMeterClient,
    metrics: &mut Metrics,
) -> Result<RawData, Error> {
    state.insert(call_hash, ProofStatus::Proving);
    let host_output =
        Host::prove(prover, call_guest_id, preflight_result).map_err(HostError::Proving)?;
    let cycles_used = host_output.cycles_used;
    let elapsed_time = host_output.elapsed_time;

    info!(
        cycles_used = cycles_used,
        elapsed_time = elapsed_time.as_millis(),
        "proving finished"
    );

    let raw_data: RawData = host_output.try_into()?;
    metrics.cycles = cycles_used;
    metrics.times.proving = metrics::elapsed_time_as_millis_u64(elapsed_time)?;

    gas_meter_client
        .refund(ComputationStage::Proving, 0)
        .await?;

    Ok(raw_data)
}

#[derive(Serialize, Clone)]
pub struct RawData {
    #[serde(with = "ProofDTO")]
    proof: Proof,
    #[serde(serialize_with = "ser_evm_call_result")]
    evm_call_result: Vec<u8>,
}

fn ser_evm_call_result<S>(evm_call_result: &[u8], state: S) -> Result<S::Ok, S::Error>
where
    S: Serializer,
{
    state.serialize_str(&evm_call_result.encode_hex_with_prefix())
}

impl TryFrom<HostOutput> for RawData {
    type Error = seal::Error;

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

fn decode_seal(seal: &[u8]) -> Result<Seal, seal::Error> {
    Ok(Seal::abi_decode(seal, true)?)
}

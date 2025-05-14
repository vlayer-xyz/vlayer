use alloy_primitives::{U256, hex::ToHexExt};
use alloy_sol_types::SolValue;
use call_engine::{CallGuestId, HostOutput, Proof, Seal};
use call_host::{Host, Prover, ProvingError, ProvingInput};
use serde::{Serialize, Serializer};
use tracing::info;

use crate::{
    gas_meter::{Client as GasMeterClient, ComputationStage, Error as GasMeterError},
    metrics::{self, Error as MetricsError, Metrics},
    ser::ProofDTO,
    v_get_proof_receipt::State,
};

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error(transparent)]
    Proving(#[from] ProvingError),
    #[error("Refunding gas: {0}")]
    RefundingGas(#[from] GasMeterError),
    #[error("Metrics: {0}")]
    Metrics(#[from] MetricsError),
    #[error("Encoding seal: {0}")]
    EncodingSeal(#[from] seal::Error),
}

pub async fn await_proving(
    prover: &Prover,
    call_guest_id: CallGuestId,
    prover_input: ProvingInput,
    gas_meter_client: &impl GasMeterClient,
    metrics: &mut Metrics,
) -> Result<RawData, Error> {
    let host_output = Host::prove(prover, call_guest_id, prover_input)?;
    let cycles_used = host_output.cycles_used;
    let elapsed_time = host_output.elapsed_time;

    info!(
        state = tracing::field::debug(State::Proving),
        cycles_used = cycles_used,
        elapsed_time = elapsed_time.as_millis(),
        "Finished stage"
    );

    let raw_data: RawData = host_output.try_into()?;
    metrics.cycles = cycles_used;
    metrics.times.proving = metrics::elapsed_time_as_millis_u64(elapsed_time)?;

    gas_meter_client
        .refund(ComputationStage::Proving, metrics.gas)
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

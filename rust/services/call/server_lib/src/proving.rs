use call_engine::CallGuestId;
use call_host::{Error as HostError, Host, PreflightResult, Prover};
use tracing::info;

use crate::{
    gas_meter::{Client as GasMeterClient, ComputationStage, Error as GasMeterError},
    handlers::{ProofStatus, RawData, SharedProofs},
    metrics::{self, Error as MetricsError, Metrics},
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

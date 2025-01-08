use call_engine::Call as EngineCall;
use call_host::{Error as HostError, Host, PreflightResult};
use tracing::info;

use crate::{
    gas_meter::{Client as GasMeterClient, ComputationStage, Error as GasMeterError},
    handlers::{ProofStatus, SharedProofs},
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
}

pub async fn await_preflight(
    host: Host,
    state: &SharedProofs,
    call: EngineCall,
    call_hash: CallHash,
    gas_meter_client: &impl GasMeterClient,
    metrics: &mut Metrics,
) -> Result<PreflightResult, Error> {
    state.insert(call_hash, ProofStatus::Preflight);
    let result = host.preflight(call).await.map_err(HostError::Preflight)?;
    let gas_used = result.gas_used;
    let elapsed_time = result.elapsed_time;

    info!(
        gas_used = gas_used,
        elapsed_time = elapsed_time.as_millis(),
        "preflight finished",
    );

    gas_meter_client
        .refund(ComputationStage::Preflight, gas_used)
        .await?;

    metrics.gas = gas_used;
    metrics.times.preflight = metrics::elapsed_time_as_millis_u64(elapsed_time)?;

    Ok(result)
}

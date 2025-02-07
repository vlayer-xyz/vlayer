use call_engine::Call as EngineCall;
use call_host::{Host, PreflightError, PreflightResult, ProvingInput};
use tracing::info;

use crate::{
    gas_meter::{Client as GasMeterClient, ComputationStage, Error as GasMeterError},
    metrics::{self, Error as MetricsError, Metrics},
    v_get_proof_receipt::State,
};

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error(transparent)]
    Preflight(#[from] PreflightError),
    #[error("Gas meter error: {0}")]
    GasMeter(#[from] GasMeterError),
    #[error("Metrics error: {0}")]
    Metrics(#[from] MetricsError),
}

pub async fn await_preflight(
    host: Host,
    call: EngineCall,
    gas_meter_client: &impl GasMeterClient,
    metrics: &mut Metrics,
) -> Result<ProvingInput, Error> {
    let PreflightResult {
        host_output,
        input,
        gas_used,
        elapsed_time,
        metadata,
    } = host.preflight(call).await?;

    info!(
        state = tracing::field::debug(State::Preflight),
        gas_used = gas_used,
        elapsed_time = elapsed_time.as_millis(),
        "Finished stage",
    );
    info!("Gathered metadata: {metadata:#?}");

    gas_meter_client
        .refund(ComputationStage::Preflight, gas_used)
        .await?;
    gas_meter_client.send_metadata(metadata).await?;

    metrics.gas = gas_used;
    metrics.times.preflight = metrics::elapsed_time_as_millis_u64(elapsed_time)?;

    Ok(ProvingInput::new(host_output, input))
}

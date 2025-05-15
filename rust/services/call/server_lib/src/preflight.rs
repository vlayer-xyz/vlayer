use call_common::Metadata;
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
    #[error("Refunding gas: {0}")]
    RefundingGas(#[from] GasMeterError),
    #[error("Metrics: {0}")]
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

    for meta in &metadata {
        match meta {
            Metadata::Precompile(x) => {
                info!(
                    metadata = "precompile",
                    precompile_tag = tracing::field::debug(x.tag),
                    precompile_calldata_length = x.calldata_length
                )
            }
            Metadata::StartChain(x) => info!(metadata = "start_chain", start_chain = x),
            Metadata::SetChain(x) => {
                info!(
                    metadata = "set_chain",
                    set_chain_chain_id = x.chain_id,
                    set_chain_block_number = x.block_number
                )
            }
            Metadata::SetBlock(x) => {
                info!(
                    metadata = "set_block",
                    set_block_chain_id = x.chain_id,
                    set_block_block_number = x.block_number
                )
            }
        }
    }

    gas_meter_client
        .refund(ComputationStage::Preflight, gas_used)
        .await?;
    gas_meter_client.send_metadata(metadata).await?;

    metrics.gas = gas_used;
    metrics.times.preflight = metrics::elapsed_time_as_millis_u64(elapsed_time)?;

    Ok(ProvingInput::new(host_output, input))
}

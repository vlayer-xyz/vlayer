use std::time::Duration;

use call_common::Metadata;
use call_engine::Call as EvmCall;
use call_host::{Host, PreflightError, PreflightResult};
use tracing::info;

use crate::{
    gas_meter::Error as GasMeterError,
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
    #[error("Preflight operation timed out after {timeout:?}")]
    Timeout { timeout: std::time::Duration },
}

pub async fn await_preflight(
    host: Host,
    evm_call: EvmCall,
    metrics: &mut Metrics,
    timeout: Duration,
) -> Result<PreflightResult, Error> {
    dbg!("Starting preflight operation", timeout);

    let preflight_future = host.preflight(evm_call);

    let result @ PreflightResult {
        gas_used,
        elapsed_time,
        ..
    } = match tokio::time::timeout(timeout, preflight_future).await {
        Ok(result) => {
            let result = result?;
            dbg!("Preflight completed successfully");
            result
        }
        Err(_) => {
            dbg!("Preflight timed out", timeout);
            return Err(Error::Timeout { timeout });
        }
    };

    info!(
        state = tracing::field::debug(State::Preflight),
        gas_used = gas_used,
        elapsed_time = elapsed_time.as_millis(),
        "Finished stage",
    );

    for meta in result.metadata.clone() {
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

    metrics.gas = gas_used;
    metrics.times.preflight = metrics::elapsed_time_as_millis_u64(elapsed_time)?;

    Ok(result)
}

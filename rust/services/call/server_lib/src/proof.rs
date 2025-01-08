use call_engine::Call as EngineCall;
use call_host::Host;
use jsonrpsee::types::error::{self as jrpcerror, ErrorObjectOwned};
use tracing::info;

use crate::{
    chain_proof::{self, Config as ChainProofConfig, Error as ChainProofError},
    gas_meter::Client as GasMeterClient,
    handlers::{ProofReceipt, SharedProofs},
    metrics::Metrics,
    preflight::{self, Error as PreflightError},
    proving::{self, Error as ProvingError},
    v_call::CallHash,
};

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("Chain proof error: {0}")]
    ChainProof(#[from] ChainProofError),
    #[error("Preflight error: {0}")]
    Preflight(#[from] PreflightError),
    #[error("Proving error: {0}")]
    Proving(#[from] ProvingError),
}

impl From<Error> for ErrorObjectOwned {
    fn from(value: Error) -> Self {
        (&value).into()
    }
}

impl From<&Error> for ErrorObjectOwned {
    fn from(error: &Error) -> Self {
        match error {
            Error::ChainProof(..) | Error::Preflight(..) | Error::Proving(..) => {
                ErrorObjectOwned::owned::<()>(
                    jrpcerror::INTERNAL_ERROR_CODE,
                    error.to_string(),
                    None,
                )
            }
        }
    }
}

pub type Result<T> = std::result::Result<T, Error>;

pub async fn generate(
    call: EngineCall,
    host: Host,
    gas_meter_client: impl GasMeterClient,
    state: SharedProofs,
    call_hash: CallHash,
    chain_proof_config: Option<ChainProofConfig>,
) -> Result<ProofReceipt> {
    let mut metrics = Metrics::default();

    let prover = host.prover();
    let call_guest_id = host.call_guest_id();

    info!("Processing call {call_hash}");

    chain_proof::await_ready(&host, &state, call_hash, chain_proof_config).await?;

    let preflight_result =
        preflight::await_preflight(host, &state, call, call_hash, &gas_meter_client, &mut metrics)
            .await?;

    let raw_data = proving::await_proving(
        &prover,
        &state,
        call_hash,
        call_guest_id,
        preflight_result,
        &gas_meter_client,
        &mut metrics,
    )
    .await?;

    Ok(ProofReceipt::new(raw_data, metrics))
}

pub use crate::proving::RawData;

use call_engine::Call as EngineCall;
use call_host::Host;
use tracing::info;

use crate::{
    chain_proof::{self, Config as ChainProofConfig, Error as ChainProofError},
    gas_meter::Client as GasMeterClient,
    handlers::{ProofStatus, SharedProofs},
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

pub async fn generate(
    call: EngineCall,
    host: Host,
    gas_meter_client: impl GasMeterClient,
    state: SharedProofs,
    call_hash: CallHash,
    chain_proof_config: Option<ChainProofConfig>,
) {
    let prover = host.prover();
    let call_guest_id = host.call_guest_id();
    let mut metrics = Metrics::default();

    let update_state = |status| {
        state.insert(call_hash, status);
    };

    info!("Generating proof for {call_hash}");

    update_state(ProofStatus::ChainProof);

    match chain_proof::await_ready(&host, chain_proof_config)
        .await
        .map_err(Error::ChainProof)
    {
        Ok(()) => update_state(ProofStatus::Preflight),
        Err(err) => {
            update_state(ProofStatus::ChainProofError(err));
            return;
        }
    }

    let preflight_result =
        match preflight::await_preflight(host, call, &gas_meter_client, &mut metrics)
            .await
            .map_err(Error::Preflight)
        {
            Ok(res) => {
                update_state(ProofStatus::Proving);
                res
            }
            Err(err) => {
                update_state(ProofStatus::PreflightError((metrics, err)));
                return;
            }
        };

    match proving::await_proving(
        &prover,
        call_guest_id,
        preflight_result,
        &gas_meter_client,
        &mut metrics,
    )
    .await
    .map_err(Error::Proving)
    {
        Ok(raw_data) => update_state(ProofStatus::Done((metrics, raw_data))),
        Err(err) => update_state(ProofStatus::ProvingError((metrics, err))),
    }
}

use call_engine::Call as EngineCall;
use call_host::Host;
use derive_new::new;
use tracing::info;

pub use crate::proving::RawData;
use crate::{
    chain_proof::{self, Config as ChainProofConfig, Error as ChainProofError},
    gas_meter::Client as GasMeterClient,
    handlers::SharedProofs,
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

pub enum ProofStatus {
    /// Proof task has just been queued
    Queued,
    /// Waiting for chain service to generate proof for the start execution location
    ChainProofPending,
    ChainProofError(Box<ProofError>),
    /// Preflight computation in progress
    PreflightPending,
    PreflightError(Box<ProofError>),
    /// Proof is being generated
    ProvingPending,
    ProvingError(Box<ProofError>),
    /// Proof generation finished
    Done(Box<ProofResult>),
}

impl ProofStatus {
    pub const fn is_err(&self) -> bool {
        matches!(
            self,
            Self::ChainProofError(..) | Self::PreflightError(..) | Self::ProvingError(..)
        )
    }

    pub fn metrics(&self) -> Metrics {
        match self {
            Self::ChainProofError(error)
            | Self::PreflightError(error)
            | Self::ProvingError(error) => error.metrics,
            Self::Done(result) => result.metrics,
            _ => Metrics::default(),
        }
    }

    pub fn data(&self) -> Option<RawData> {
        match self {
            Self::Done(result) => Some(result.data.clone()),
            _ => None,
        }
    }

    pub const fn err(&self) -> Option<&Error> {
        match self {
            Self::ChainProofError(err) | Self::PreflightError(err) | Self::ProvingError(err) => {
                Some(&err.error)
            }
            _ => None,
        }
    }
}

#[derive(new)]
pub struct ProofError {
    metrics: Metrics,
    error: Error,
}

#[derive(new)]
pub struct ProofResult {
    metrics: Metrics,
    data: RawData,
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

    update_state(ProofStatus::ChainProofPending);

    match chain_proof::await_ready(&host, chain_proof_config)
        .await
        .map_err(Error::ChainProof)
    {
        Ok(()) => update_state(ProofStatus::PreflightPending),
        Err(err) => {
            update_state(ProofStatus::ChainProofError(Box::new(ProofError::new(metrics, err))));
            return;
        }
    }

    let preflight_result =
        match preflight::await_preflight(host, call, &gas_meter_client, &mut metrics)
            .await
            .map_err(Error::Preflight)
        {
            Ok(res) => {
                update_state(ProofStatus::ProvingPending);
                res
            }
            Err(err) => {
                update_state(ProofStatus::PreflightError(Box::new(ProofError::new(metrics, err))));
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
        Ok(raw_data) => {
            update_state(ProofStatus::Done(Box::new(ProofResult::new(metrics, raw_data))))
        }
        Err(err) => {
            update_state(ProofStatus::ProvingError(Box::new(ProofError::new(metrics, err))))
        }
    }
}

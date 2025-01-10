use call_engine::Call as EngineCall;
use call_host::Host;
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

#[derive(Default)]
pub enum State {
    #[default]
    Queued,
    ChainProofPending,
    ChainProofError(Box<Error>),
    PreflightPending,
    PreflightError(Box<Error>),
    ProvingPending,
    ProvingError(Box<Error>),
    Done(Box<RawData>),
}

impl State {
    pub const fn is_err(&self) -> bool {
        matches!(
            self,
            State::ChainProofError(..) | State::PreflightError(..) | State::ProvingError(..)
        )
    }

    pub fn data(&self) -> Option<&RawData> {
        match self {
            State::Done(data) => Some(&data),
            _ => None,
        }
    }

    pub fn err(&self) -> Option<&Error> {
        match self {
            State::ChainProofError(err) | State::PreflightError(err) | State::ProvingError(err) => {
                Some(&err)
            }
            _ => None,
        }
    }
}

#[derive(Default)]
pub struct Status {
    pub state: State,
    pub metrics: Metrics,
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

    info!("Generating proof for {call_hash}");

    state
        .entry(call_hash)
        .and_modify(|res| res.state = State::ChainProofPending);

    match chain_proof::await_ready(&host, chain_proof_config)
        .await
        .map_err(Error::ChainProof)
    {
        Ok(()) => {
            state
                .entry(call_hash)
                .and_modify(|res| res.state = State::PreflightPending);
        }
        Err(err) => {
            state.entry(call_hash).and_modify(|res| {
                res.state = State::ChainProofError(err.into());
            });
            return;
        }
    }

    let preflight_result =
        match preflight::await_preflight(host, call, &gas_meter_client, &mut metrics)
            .await
            .map_err(Error::Preflight)
        {
            Ok(res) => {
                state.entry(call_hash).and_modify(|res| {
                    res.state = State::ProvingPending;
                    res.metrics = metrics;
                });
                res
            }
            Err(err) => {
                state.entry(call_hash).and_modify(|res| {
                    res.state = State::PreflightError(err.into());
                    res.metrics = metrics;
                });
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
        Ok(raw_data) => state.entry(call_hash).and_modify(|res| {
            res.state = State::Done(raw_data.into());
            res.metrics = metrics;
        }),
        Err(err) => state.entry(call_hash).and_modify(|res| {
            res.state = State::ProvingError(err.into());
            res.metrics = metrics;
        }),
    };
}

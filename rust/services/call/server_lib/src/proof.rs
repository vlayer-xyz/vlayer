use call_engine::Call as EngineCall;
use call_host::Host;
use dashmap::Entry;
use tracing::{error, info, instrument};

pub use crate::proving::RawData;
use crate::{
    gas_meter::{Client as GasMeterClient, Error as GasMeterError},
    handlers::State as AppState,
    metrics::Metrics,
    preflight::{self, Error as PreflightError},
    proving::{self, Error as ProvingError},
    v_call::CallHash,
};

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("Allocating gas: {0}")]
    AllocateGas(#[from] GasMeterError),
    #[error("Preflight: {0}")]
    Preflight(#[from] PreflightError),
    #[error("Proving: {0}")]
    Proving(#[from] ProvingError),
}

#[derive(Default)]
pub enum State {
    #[default]
    Queued,
    AllocateGasPending,
    AllocateGasError(Box<Error>),
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
            State::AllocateGasError(..) | State::PreflightError(..) | State::ProvingError(..)
        )
    }

    pub const fn data(&self) -> Option<&RawData> {
        match self {
            State::Done(data) => Some(data),
            _ => None,
        }
    }

    pub const fn err(&self) -> Option<&Error> {
        match self {
            State::AllocateGasError(err)
            | State::PreflightError(err)
            | State::ProvingError(err) => Some(err),
            _ => None,
        }
    }
}

#[derive(Default)]
pub struct Status {
    pub state: State,
    pub metrics: Metrics,
}

fn set_state(
    app_state: &AppState,
    call_hash: CallHash,
    state: State,
) -> Entry<'_, CallHash, Status> {
    app_state
        .entry(call_hash)
        .and_modify(|res| res.state = state)
}

fn set_metrics(
    entry: Entry<'_, CallHash, Status>,
    metrics: Metrics,
) -> Entry<'_, CallHash, Status> {
    entry.and_modify(|res| res.metrics = metrics)
}

#[instrument(name = "proof", skip_all, fields(hash = %call_hash))]
pub async fn generate(
    call: EngineCall,
    host: Host,
    gas_meter_client: impl GasMeterClient,
    state: AppState,
    call_hash: CallHash,
) {
    let prover = host.prover();
    let call_guest_id = host.call_guest_id();
    let mut metrics = Metrics::default();

    info!("Generating proof");

    set_state(&state, call_hash, State::AllocateGasPending);

    match gas_meter_client
        .allocate(call.gas_limit)
        .await
        .map_err(Error::AllocateGas)
    {
        Ok(()) => {
            set_state(&state, call_hash, State::PreflightPending);
        }
        Err(err) => {
            error!("Gas meter failed with error: {err}");
            set_state(&state, call_hash, State::AllocateGasError(err.into()));
            return;
        }
    };

    let preflight_result =
        match preflight::await_preflight(host, call, &gas_meter_client, &mut metrics)
            .await
            .map_err(Error::Preflight)
        {
            Ok(res) => {
                let entry = set_state(&state, call_hash, State::ProvingPending);
                set_metrics(entry, metrics);
                res
            }
            Err(err) => {
                error!("Preflight failed with error: {err}");
                let entry = set_state(&state, call_hash, State::PreflightError(err.into()));
                set_metrics(entry, metrics);
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
            let entry = set_state(&state, call_hash, State::Done(raw_data.into()));
            set_metrics(entry, metrics);
        }
        Err(err) => {
            error!("Proving failed with error: {err}");
            let entry = set_state(&state, call_hash, State::ProvingError(err.into()));
            set_metrics(entry, metrics);
        }
    };
}

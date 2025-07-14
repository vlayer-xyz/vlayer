use call_engine::Call as EngineCall;
use call_host::{CycleEstimator, CycleEstimatorError, Host, ProvingInput, Risc0CycleEstimator};
use dashmap::Entry;
use tracing::{error, info, instrument, warn};

pub use crate::proving::RawData;
use crate::{
    gas_meter::{Client as GasMeterClient, Error as GasMeterError},
    handlers::State as AppState,
    metrics::Metrics,
    preflight::{self, Error as PreflightError},
    proving::{self, Error as ProvingError},
    v_call::CallHash,
};

const CYCLES_PER_VGAS: u64 = 1_000_000;

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("Allocating gas: {0}")]
    AllocateGasRpc(#[from] GasMeterError),
    #[error("Your gas balance is insufficient to allocate given gas_limit of {gas_limit}.")]
    AllocateGasInsufficientBalance { gas_limit: u64 },
    #[error("Preflight: {0}")]
    Preflight(#[from] PreflightError),
    #[error("Can can consume at most {gas_limit} EVM gas.")]
    PreflightGasLimitExceeded { gas_limit: u64 },
    #[error("Estimating cycles: {0}")]
    EstimatingCycles(#[from] CycleEstimatorError),
    #[error("Insufficient gas: provided {provided}, estimated cycles: {estimated}")]
    InsufficientGas { provided: u64, estimated: u64 },
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
    EstimatingCyclesPending,
    EstimatingCyclesError(Box<Error>),
    ProvingPending,
    ProvingError(Box<Error>),
    Done(Box<RawData>),
}

impl State {
    pub const fn is_err(&self) -> bool {
        matches!(
            self,
            State::AllocateGasError(..)
                | State::PreflightError(..)
                | State::ProvingError(..)
                | State::EstimatingCyclesError(..)
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
    vgas_limit: u64,
    state: AppState,
    call_hash: CallHash,
) {
    let prover = host.prover();
    let call_guest_id = host.call_guest_id();
    let mut metrics = Metrics::default();
    let cycles_limit = vgas_limit * CYCLES_PER_VGAS;

    info!("Generating proof");

    set_state(&state, call_hash, State::AllocateGasPending);

    match gas_meter_client.allocate(vgas_limit).await {
        Ok(()) => {
            set_state(&state, call_hash, State::PreflightPending);
        }
        Err(err) => {
            let state_value = if err.is_insufficient_gas_balance() {
                State::AllocateGasError(
                    Error::AllocateGasInsufficientBalance {
                        gas_limit: vgas_limit,
                    }
                    .into(),
                )
            } else {
                error!("Gas meter failed with error: {err}");
                State::AllocateGasError(Error::AllocateGasRpc(err).into())
            };
            set_state(&state, call_hash, state_value);
            return;
        }
    };

    let gas_limit = call.gas_limit;
    let preflight_result =
        match preflight::await_preflight(host, call, &gas_meter_client, &mut metrics).await {
            Ok(res) => {
                let entry = set_state(&state, call_hash, State::EstimatingCyclesPending);
                set_metrics(entry, metrics);
                res
            }
            Err(err) => {
                let state_value = match err {
                    preflight::Error::Preflight(preflight_err)
                        if preflight_err.is_gas_limit_exceeded() =>
                    {
                        error!("Preflight gas limit exceeded!");
                        State::PreflightError(Error::PreflightGasLimitExceeded { gas_limit }.into())
                    }
                    preflight::Error::Preflight(preflight_err) => {
                        error!("Preflight failed with error: {preflight_err}");
                        State::PreflightError(
                            Error::Preflight(preflight::Error::Preflight(preflight_err)).into(),
                        )
                    }
                    other_err => {
                        error!("Preflight failed with error: {other_err}");
                        State::PreflightError(Error::Preflight(other_err).into())
                    }
                };
                let entry = set_state(&state, call_hash, state_value);
                set_metrics(entry, metrics);
                return;
            }
        };

    let estimation_start = std::time::Instant::now();

    let estimated_cycles =
        match Risc0CycleEstimator.estimate(&preflight_result.input, preflight_result.guest_elf) {
            Ok(result) => {
                info!(estimated_cycles = result, "Cycle estimation");
                result
            }
            Err(err) => {
                error!("Cycle estimation failed with error: {err}");
                let entry = set_state(
                    &state,
                    call_hash,
                    State::EstimatingCyclesError(Box::new(Error::EstimatingCycles(err))),
                );
                set_metrics(entry, metrics);
                return;
            }
        };

    let elapsed = estimation_start.elapsed();
    info!(estimating_cycles_elapsed_time = ?elapsed, "Cycle estimation lasted");

    if cycles_limit < estimated_cycles {
        warn!(
            "Insufficient gas: provided {} vgas ({} cycles), estimated cycles: {}",
            vgas_limit, cycles_limit, estimated_cycles
        );
        let entry = set_state(
            &state,
            call_hash,
            State::EstimatingCyclesError(Box::new(Error::InsufficientGas {
                provided: cycles_limit,
                estimated: estimated_cycles,
            })),
        );
        set_metrics(entry, metrics);
        return;
    }

    set_state(&state, call_hash, State::ProvingPending);

    let proving_input = ProvingInput::new(preflight_result.host_output, preflight_result.input);
    match proving::await_proving(
        &prover,
        call_guest_id,
        proving_input,
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

use call_common::Metadata;
use call_engine::Call as EngineCall;
use call_host::{CycleEstimator, CycleEstimatorError, Host, ProvingInput, Risc0CycleEstimator};
use dashmap::Entry;
use tracing::{error, info, instrument, warn};

pub use crate::proving::RawData;
use crate::{
    gas_meter::{Client as GasMeterClient, ComputationStage, Error as GasMeterError},
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
    #[error("Your vgas balance is insufficient to allocate given vgas_limit of {vgas_limit}.")]
    AllocateGasInsufficientBalance { vgas_limit: u64 },
    #[error("Preflight: {0}")]
    Preflight(#[from] PreflightError),
    #[error("EVM gas limit {evm_gas_limit} exceeded.")]
    PreflightEvmGasLimitExceeded { evm_gas_limit: u64 },
    #[error("Estimating cycles: {0}")]
    EstimatingCycles(#[from] CycleEstimatorError),
    #[error("Insufficient vgas_limit: provided {provided}, estimated vgas: {estimated}")]
    InsufficientVgas { provided: u64, estimated: u64 },
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
            | State::EstimatingCyclesError(err)
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

fn allocate_error_to_state(err: GasMeterError, vgas_limit: u64) -> State {
    if err.is_insufficient_gas_balance() {
        return State::AllocateGasError(
            Error::AllocateGasInsufficientBalance { vgas_limit }.into(),
        );
    }
    error!("Gas meter failed with error: {err}");
    State::AllocateGasError(Error::AllocateGasRpc(err).into())
}

fn preflight_error_to_state(err: PreflightError, evm_gas_limit: u64) -> State {
    if let preflight::Error::Preflight(ref preflight_err) = err {
        if preflight_err.is_gas_limit_exceeded() {
            error!("Preflight gas limit exceeded!");
            return State::PreflightError(
                Error::PreflightEvmGasLimitExceeded { evm_gas_limit }.into(),
            );
        }
    }

    error!("Preflight failed with error: {err}");
    State::PreflightError(Error::Preflight(err).into())
}

const fn to_vgas(cycles: u64) -> u64 {
    cycles.div_ceil(CYCLES_PER_VGAS)
}

const fn to_cycles(vgas: u64) -> u64 {
    vgas * CYCLES_PER_VGAS
}

/// Attempts to allocate vgas from the gas meter client.
///
/// On success, updates the application state to PreflightPending and returns `true`.
/// On failure, logs the error, updates the application state with the allocation error,
/// and returns `false` to signal that the caller should abort execution.
///
/// # Returns
/// - `true` if vgas allocation succeeded
/// - `false` if vgas allocation failed (caller should return immediately)
async fn allocate_vgas(
    gas_meter_client: &impl GasMeterClient,
    vgas_limit: u64,
    app_state: &AppState,
    call_hash: CallHash,
) -> bool {
    set_state(app_state, call_hash, State::AllocateGasPending);

    match gas_meter_client.allocate(vgas_limit).await {
        Ok(()) => {
            set_state(app_state, call_hash, State::PreflightPending);
            true
        }
        Err(err) => {
            let state = allocate_error_to_state(err, vgas_limit);
            set_state(app_state, call_hash, state);
            false
        }
    }
}

/// Executes the preflight phase and updates application state.
///
/// On success, updates the application state to EstimatingCyclesPending and returns the preflight result.
/// On failure, logs the error, updates the application state with the preflight error,
/// and returns `None` to signal that the caller should abort execution.
///
/// # Returns
/// - `Some(preflight_result)` if preflight succeeded
/// - `None` if preflight failed (caller should return immediately)
async fn preflight(
    host: Host,
    call: EngineCall,
    evm_gas_limit: u64,
    app_state: &AppState,
    call_hash: CallHash,
    metrics: &mut Metrics,
) -> Option<call_host::PreflightResult> {
    match preflight::await_preflight(host, call, metrics).await {
        Ok(res) => {
            let entry = set_state(app_state, call_hash, State::EstimatingCyclesPending);
            set_metrics(entry, *metrics);
            Some(res)
        }
        Err(err) => {
            let state = preflight_error_to_state(err, evm_gas_limit);
            let entry = set_state(app_state, call_hash, state);
            set_metrics(entry, *metrics);
            None
        }
    }
}

/// Attempts to refund gas to the gas meter client after preflight computation.
///
/// On success, logs the refund and returns `true`.
/// On failure, logs the error, updates the application state with the error,
/// and returns `false` to signal that the caller should abort execution.
///
/// # Returns
/// - `true` if the refund succeeded
/// - `false` if the refund failed (caller should return immediately)
async fn refund(
    gas_meter_client: &impl GasMeterClient,
    estimated_vgas: u64,
    app_state: AppState,
    call_hash: CallHash,
    metrics: Metrics,
) -> bool {
    match gas_meter_client
        .refund(ComputationStage::Preflight, estimated_vgas)
        .await
    {
        Ok(()) => {
            info!("Preflight refund succeeded for {estimated_vgas} vgas");
            true
        }
        Err(err) => {
            error!("Preflight refund failed with error: {err}");
            let entry = set_state(
                &app_state,
                call_hash,
                State::PreflightError(Error::AllocateGasRpc(err).into()),
            );
            set_metrics(entry, metrics);
            false
        }
    }
}

/// Attempts to send metadata to the gas meter client.
///
/// On success, logs the operation and returns `true`.
/// On failure, logs the error, updates the application state with the error,
/// and returns `false` to signal that the caller should abort execution.
///
/// # Returns
/// - `true` if sending metadata succeeded
/// - `false` if sending metadata failed (caller should return immediately)
async fn send_metadata(
    gas_meter_client: &impl GasMeterClient,
    metadata: Box<[Metadata]>,
    app_state: AppState,
    call_hash: CallHash,
    metrics: Metrics,
) -> bool {
    match gas_meter_client.send_metadata(metadata).await {
        Ok(()) => {
            info!("Send metadata succeeded");
            true
        }
        Err(err) => {
            error!("Send metadata failed with error: {err}");
            let entry = set_state(
                &app_state,
                call_hash,
                State::PreflightError(Error::AllocateGasRpc(err).into()),
            );
            set_metrics(entry, metrics);
            false
        }
    }
}

/// Validates that the provided vgas limit is sufficient for the estimated vgas requirement.
///
/// On success, returns `true` and continues execution.
/// On failure, logs a warning, updates the application state with an insufficient vgas error,
/// and returns `false` to signal that the caller should abort execution.
///
/// # Returns
/// - `true` if vgas limit is sufficient
/// - `false` if vgas limit is insufficient (caller should return immediately)
fn validate_vgas_limit(
    vgas_limit: u64,
    estimated_vgas: u64,
    estimated_cycles: u64,
    app_state: &AppState,
    call_hash: CallHash,
    metrics: Metrics,
) -> bool {
    let cycles_limit = to_cycles(vgas_limit);

    if vgas_limit <= estimated_vgas {
        warn!(
            "Insufficient vgas_limit: provided {} vgas ({} cycles), estimated vgas: {} ({} cycles)",
            vgas_limit, cycles_limit, estimated_vgas, estimated_cycles
        );
        let entry = set_state(
            app_state,
            call_hash,
            State::EstimatingCyclesError(Box::new(Error::InsufficientVgas {
                provided: vgas_limit,
                estimated: estimated_vgas,
            })),
        );
        set_metrics(entry, metrics);
        false
    } else {
        true
    }
}

#[instrument(name = "proof", skip_all, fields(hash = %call_hash))]
pub async fn generate(
    call: EngineCall,
    host: Host,
    gas_meter_client: impl GasMeterClient,
    vgas_limit: u64,
    app_state: AppState,
    call_hash: CallHash,
) {
    let prover = host.prover();
    let call_guest_id = host.call_guest_id();
    let mut metrics = Metrics::default();

    info!("Generating proof");

    if !allocate_vgas(&gas_meter_client, vgas_limit, &app_state, call_hash).await {
        return;
    }

    let evm_gas_limit = call.gas_limit;
    let Some(preflight_result) =
        preflight(host, call, evm_gas_limit, &app_state, call_hash, &mut metrics)
            .await
    else {
        return;
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
                    &app_state,
                    call_hash,
                    State::EstimatingCyclesError(Box::new(Error::EstimatingCycles(err))),
                );
                set_metrics(entry, metrics);
                return;
            }
        };

    let elapsed = estimation_start.elapsed();
    info!(estimating_cycles_elapsed_time = ?elapsed, "Cycle estimation lasted");

    let estimated_vgas = to_vgas(estimated_cycles);
    metrics.gas = estimated_vgas;

    if !refund(&gas_meter_client, estimated_vgas, app_state.clone(), call_hash, metrics).await {
        return;
    }

    if !send_metadata(
        &gas_meter_client,
        preflight_result.metadata,
        app_state.clone(),
        call_hash,
        metrics,
    )
    .await
    {
        return;
    }

    if !validate_vgas_limit(
        vgas_limit,
        estimated_vgas,
        estimated_cycles,
        &app_state,
        call_hash,
        metrics,
    ) {
        return;
    }

    set_state(&app_state, call_hash, State::ProvingPending);

    let proving_input = ProvingInput::new(preflight_result.host_output, preflight_result.input);
    match proving::await_proving(
        &prover,
        call_guest_id,
        proving_input,
        &gas_meter_client,
        &mut metrics,
        estimated_vgas,
    )
    .await
    .map_err(Error::Proving)
    {
        Ok(raw_data) => {
            let entry = set_state(&app_state, call_hash, State::Done(raw_data.into()));
            set_metrics(entry, metrics);
        }
        Err(err) => {
            error!("Proving failed with error: {err}");
            let entry = set_state(&app_state, call_hash, State::ProvingError(err.into()));
            set_metrics(entry, metrics);
        }
    };
}

use call_host::CycleEstimatorError;
use dashmap::Entry;
use tracing::error;

pub use crate::proving::RawData;
use crate::{
    gas_meter::Error as GasMeterError,
    metrics::Metrics,
    preflight::{self, Error as PreflightError},
    proof::state::State,
    proving::Error as ProvingError,
    v_call::CallHash,
};

pub mod generator;
pub mod state;

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

const CYCLES_PER_VGAS: u64 = 1_000_000;

#[derive(Default)]
pub struct Status {
    pub state: State,
    pub metrics: Metrics,
}

fn set_metrics(
    entry: Entry<'_, CallHash, Status>,
    metrics: Metrics,
) -> Entry<'_, CallHash, Status> {
    entry.and_modify(|status| status.metrics = metrics)
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

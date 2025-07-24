use dashmap::Entry;

use crate::{
    handlers::State as AppState,
    proof::{Error, RawData, Status},
    v_call::CallHash,
};

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

pub fn set_state(
    app_state: &AppState,
    call_hash: CallHash,
    state: State,
) -> Entry<'_, CallHash, Status> {
    app_state
        .entry(call_hash)
        .and_modify(|res| res.state = state)
}

use jsonrpsee::types::error::{self as jrpcerror, ErrorObjectOwned};
use serde::{Deserialize, Serialize};

use crate::{
    metrics::Metrics,
    proof::{Error as ProofError, State as ProofState, Status as ProofStatus},
    proving::RawData,
    v_call::CallHash,
};

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum State {
    Queued,
    ChainProof,
    Preflight,
    Proving,
    Done,
}

pub type Result<T> = std::result::Result<T, Error>;

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("Hash not found: {0}")]
    HashNotFound(CallHash),
}

impl From<Error> for ErrorObjectOwned {
    fn from(error: Error) -> Self {
        match error {
            Error::HashNotFound(..) => ErrorObjectOwned::owned::<()>(
                jrpcerror::INVALID_REQUEST_CODE,
                error.to_string(),
                None,
            ),
        }
    }
}

impl Default for State {
    fn default() -> Self {
        Self::Queued
    }
}

impl From<&ProofState> for State {
    fn from(value: &ProofState) -> Self {
        match value {
            ProofState::Queued => Self::Queued,
            ProofState::ChainProofPending | ProofState::ChainProofError(..) => Self::ChainProof,
            ProofState::PreflightPending | ProofState::PreflightError(..) => Self::Preflight,
            ProofState::ProvingPending | ProofState::ProvingError(..) => Self::Proving,
            ProofState::Done(..) => Self::Done,
        }
    }
}

#[derive(Clone, Serialize, Default)]
pub struct CallResult {
    pub state: State,
    pub status: u8,
    pub metrics: Metrics,
    pub data: Option<RawData>,
    pub error: Option<String>,
}

impl From<&ProofStatus> for CallResult {
    fn from(value: &ProofStatus) -> Self {
        let state: State = (&value.state).into();
        let status = if value.state.is_err() { 0 } else { 1 };
        let data = value.state.data().cloned();
        let error = value.state.err().map(ProofError::to_string);
        let metrics = value.metrics;
        Self {
            state,
            status,
            metrics,
            data,
            error,
        }
    }
}

use jsonrpsee::types::error::{self as jrpcerror, ErrorObjectOwned};
use serde::{Deserialize, Serialize};

use crate::{handlers::ProofStatus, metrics::Metrics, proof, proving::RawData, v_call::CallHash};

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum State {
    Queued,
    ChainProof,
    ChainProofError,
    Preflight,
    PreflightError,
    Proving,
    ProvingError,
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

impl From<&ProofStatus> for State {
    fn from(value: &ProofStatus) -> Self {
        match value {
            ProofStatus::Queued => Self::Queued,
            ProofStatus::ChainProof => Self::ChainProof,
            ProofStatus::ChainProofError(..) => Self::ChainProofError,
            ProofStatus::Preflight => Self::Preflight,
            ProofStatus::PreflightError(..) => Self::PreflightError,
            ProofStatus::Proving => Self::Proving,
            ProofStatus::ProvingError(..) => Self::ProvingError,
            ProofStatus::Done(..) => Self::Done,
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
        let state: State = value.into();
        let status = if value.is_err() { 0 } else { 1 };
        let data = value.data();
        let error = value.err().map(proof::Error::to_string);
        let metrics = value.metrics();
        Self {
            state,
            status,
            metrics,
            data,
            error,
        }
    }
}

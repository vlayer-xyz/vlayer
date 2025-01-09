use std::convert::TryFrom;

use jsonrpsee::types::error::{self as jrpcerror, ErrorObjectOwned};
use serde::{Deserialize, Serialize};

use crate::{
    handlers::{ProofReceipt, ProofStatus},
    v_call::CallHash,
};

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum Status {
    Queued,
    WaitingForChainProof,
    Preflight,
    Proving,
    Ready,
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

impl Default for Status {
    fn default() -> Self {
        Self::Queued
    }
}

impl From<&ProofStatus> for Status {
    fn from(value: &ProofStatus) -> Self {
        match value {
            ProofStatus::Queued => Self::Queued,
            ProofStatus::WaitingForChainProof => Self::WaitingForChainProof,
            ProofStatus::Preflight => Self::Preflight,
            ProofStatus::Proving => Self::Proving,
            ProofStatus::Ready(..) => Self::Ready,
        }
    }
}

#[derive(Clone, Serialize, Default)]
pub struct CallResult {
    pub status: Status,
    pub receipt: Option<ProofReceipt>,
}

impl TryFrom<&ProofStatus> for CallResult {
    type Error = ErrorObjectOwned;

    fn try_from(value: &ProofStatus) -> std::result::Result<Self, Self::Error> {
        let status: Status = value.into();
        let receipt: Option<ProofReceipt> = match value {
            ProofStatus::Ready(Ok(receipt)) => Some(receipt.clone()),
            ProofStatus::Ready(Err(err)) => return Err(err.into()),
            _ => None,
        };
        Ok(Self { status, receipt })
    }
}

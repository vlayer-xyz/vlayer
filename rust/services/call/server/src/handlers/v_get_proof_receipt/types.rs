use std::convert::TryFrom;

use jsonrpsee::types::error::ErrorObjectOwned;
use serde::{Deserialize, Serialize};

use crate::handlers::{ProofReceipt, ProofStatus};

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum Status {
    Queued,
    WaitingForChainProof,
    Preflight,
    Proving,
    Ready,
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

    fn try_from(value: &ProofStatus) -> Result<Self, Self::Error> {
        let status: Status = value.into();
        let receipt: Option<ProofReceipt> = match value {
            ProofStatus::Ready(Ok(receipt)) => Some(receipt.clone()),
            ProofStatus::Ready(Err(err)) => return Err(err.into()),
            _ => None,
        };
        Ok(Self { status, receipt })
    }
}

use dashmap::Entry;
use tracing::info;
use types::{CallResult, RawData, Status};

use super::SharedProofs;
use crate::{error::AppError, v_call::CallHash};

pub mod types;

pub fn v_get_proof_receipt(proofs: &SharedProofs, hash: CallHash) -> Result<CallResult, AppError> {
    info!("v_get_proof_receipt => {hash:#?}");

    let entry = proofs.entry(hash);
    let inner = match entry {
        Entry::Occupied(inner) => inner,
        Entry::Vacant(..) => {
            info!("Hash not found: {hash}");
            return Err(AppError::HashNotFound(hash));
        }
    };
    let status: Status = inner.get().into();
    let data: Option<RawData> = match status {
        Status::Ready => inner
            .remove()
            .into_ready()
            .transpose()?
            .map(TryInto::try_into)
            .transpose()?,
        _ => None,
    };

    info!("Current status {:?}", status);
    Ok(CallResult::new(status, data))
}

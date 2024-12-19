use tracing::info;
use types::{CallResult, RawData, Status};

use super::SharedProofs;
use crate::{error::AppError, v_call::CallHash};

pub mod types;

pub fn v_get_proof_receipt(proofs: &SharedProofs, hash: CallHash) -> Result<CallResult, AppError> {
    info!("v_get_proof_receipt => {hash:#?}");

    let status: Status = proofs
        .get(&hash)
        .map(|entry| entry.value().into())
        .ok_or(AppError::HashNotFound(hash))?;

    let data: Option<RawData> = proofs
        .remove_if(&hash, |_, value| value.is_ready())
        .and_then(|(_, status)| status.into_ready())
        .transpose()?
        .map(TryInto::try_into)
        .transpose()?;

    info!("Current status {:?}", status);
    Ok(CallResult::new(status, data))
}

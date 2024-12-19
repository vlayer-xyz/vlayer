use tracing::info;
use types::CallResult;

use super::SharedProofs;
use crate::{error::AppError, v_call::CallHash};

pub mod types;

pub fn v_get_proof_receipt(proofs: &SharedProofs, hash: CallHash) -> Result<CallResult, AppError> {
    info!("v_get_proof_receipt => {hash:#?}");
    let result = proofs
        .remove(&hash)
        .map_or(Ok(CallResult::default()), |(_, status)| status.try_into())?;
    info!("Current status {:?}", result.status);
    Ok(result)
}

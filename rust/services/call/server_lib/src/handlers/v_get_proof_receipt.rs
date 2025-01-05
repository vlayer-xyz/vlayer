use std::ops::Deref;

use jsonrpsee::types::error::ErrorObjectOwned;
use tracing::info;
use types::CallResult;

use super::SharedProofs;
use crate::{error::AppError, v_call::CallHash};

pub mod types;

pub fn v_get_proof_receipt(
    proofs: &SharedProofs,
    hash: CallHash,
) -> Result<CallResult, ErrorObjectOwned> {
    info!("v_get_proof_receipt => {hash:#?}");
    proofs
        .get(&hash)
        .ok_or(AppError::HashNotFound(hash))?
        .deref()
        .try_into()
}

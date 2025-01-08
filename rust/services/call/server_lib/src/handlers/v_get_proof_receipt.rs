use std::ops::Deref;

use jsonrpsee::types::error::ErrorObjectOwned;
use tracing::info;
use types::{CallResult, Error};

use super::SharedProofs;
use crate::v_call::CallHash;

pub mod types;

pub fn v_get_proof_receipt(
    proofs: &SharedProofs,
    hash: CallHash,
) -> Result<CallResult, ErrorObjectOwned> {
    info!("v_get_proof_receipt => {hash:#?}");
    proofs
        .get(&hash)
        .ok_or(Error::HashNotFound(hash))?
        .deref()
        .try_into()
}

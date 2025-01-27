use std::ops::Deref;

use tracing::info;
use types::{CallResult, Error, Result};

use super::SharedProofs;
use crate::v_call::CallHash;

pub mod types;

pub fn v_get_proof_receipt(proofs: &SharedProofs, hash: CallHash) -> Result<CallResult> {
    info!("v_get_proof_receipt => {hash:#?}");
    Ok(proofs
        .get(&hash)
        .ok_or(Error::HashNotFound(hash))?
        .deref()
        .into())
}

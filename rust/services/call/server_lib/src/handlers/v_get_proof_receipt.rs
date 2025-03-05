use std::ops::Deref;

use tracing::{info, instrument};
use types::{CallResult, Error, Result};

use super::Proofs;
use crate::v_call::CallHash;

pub mod types;

#[instrument(name = "proof", skip_all, fields(hash = %hash))]
pub fn v_get_proof_receipt(proofs: &Proofs, hash: CallHash) -> Result<CallResult> {
    info!("Getting proof receipt");
    Ok(proofs
        .get(&hash)
        .ok_or(Error::HashNotFound(hash))?
        .deref()
        .into())
}

use std::ops::Deref;

use tracing::{info, info_span};
use types::{CallResult, Error, Result};

use super::SharedProofs;
use crate::v_call::CallHash;

pub mod types;

pub fn v_get_proof_receipt(proofs: &SharedProofs, hash: CallHash) -> Result<CallResult> {
    let span = info_span!("proof", hash = tracing::field::display(hash));
    let _enter = span.enter();

    info!("Getting proof receipt");

    Ok(proofs
        .get(&hash)
        .ok_or(Error::HashNotFound(hash))?
        .deref()
        .into())
}

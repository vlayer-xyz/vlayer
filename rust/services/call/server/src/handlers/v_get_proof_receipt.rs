use tracing::info;
use types::CallResult;

use super::SharedState;
use crate::{error::AppError, v_call::CallHash};

pub mod types;

pub fn v_get_proof_receipt(state: &SharedState, hash: CallHash) -> Result<CallResult, AppError> {
    info!("v_get_proof_receipt => {hash:#?}");
    state
        .remove(&hash)
        .map(|(_, res)| res)
        .transpose()
        .and_then(CallResult::from_maybe_output)
}

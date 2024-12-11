use serde::{Deserialize, Serialize};
use tracing::info;
use types::CallResult;

use super::SharedState;
use crate::{error::AppError, handlers::ProofStatus, v_call::CallHash};

pub mod types;

#[derive(Deserialize, Serialize, Debug)]
pub struct Params {
    hash: CallHash,
}

pub async fn v_get_proof_receipt(
    state: SharedState,
    params: Params,
) -> Result<CallResult, AppError> {
    info!("v_get_proof_receipt => {params:#?}");
    state
        .remove(&params.hash)
        .and_then(|(_, status)| match status {
            ProofStatus::Ready(res) => Some(res),
            _ => None,
        })
        .transpose()
        .and_then(CallResult::from_maybe_output)
}

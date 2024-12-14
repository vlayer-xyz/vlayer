use serde::{Deserialize, Serialize};
use tracing::info;
use types::CallResult;

use super::SharedState;
use crate::{error::AppError, v_call::CallHash};

pub mod types;

#[derive(Deserialize, Serialize, Debug)]
pub struct Params {
    pub hash: CallHash,
}

pub async fn v_get_proof_receipt(
    state: SharedState,
    params: Params,
) -> Result<CallResult, AppError> {
    info!("v_get_proof_receipt => {params:#?}");
    state
        .remove(&params.hash)
        .map(|(_, res)| res)
        .transpose()
        .and_then(CallResult::from_maybe_output)
}

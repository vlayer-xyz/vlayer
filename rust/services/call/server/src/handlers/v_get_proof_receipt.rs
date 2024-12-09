use serde::{Deserialize, Serialize};
use tracing::info;
use types::{CallResult, RawData};

use super::SharedState;
use crate::{error::AppError, v_call::CallHash};

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
        .map_or(Ok(CallResult::pending()), |(_, res)| {
            res.and_then(|host_output| Ok(CallResult::done(RawData::try_new(host_output)?)))
        })
}

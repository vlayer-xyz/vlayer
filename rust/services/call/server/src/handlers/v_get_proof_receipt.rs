use serde::{Deserialize, Serialize};
use tracing::info;
use types::{CallResult, RawData, Status};

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
    if !state.read().contains_key(&params.hash) {
        return Ok(CallResult::new(Status::Pending, None));
    }
    state
        .write()
        .remove(&params.hash)
        .expect("hash must exist")
        .and_then(|host_output| {
            Ok(CallResult::new(Status::Done, Some(RawData::try_new(host_output)?)))
        })
}

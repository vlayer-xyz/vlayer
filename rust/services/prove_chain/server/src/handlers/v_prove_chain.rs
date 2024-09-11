use serde::{Deserialize, Serialize};
use types::ValidatedParams;

use crate::error::AppError;

pub mod types;

#[derive(Deserialize, Serialize)]
pub struct Params {
    block_hashes: Vec<String>,
}

#[derive(Serialize, Debug)]
pub struct ChainProof;

pub async fn v_prove_chain(params: Params) -> Result<ChainProof, AppError> {
    let params: ValidatedParams = params.try_into()?;
    if params.block_hashes.is_empty() {
        Err(AppError::NoBlockHashes)
    } else {
        todo!();
    }
}

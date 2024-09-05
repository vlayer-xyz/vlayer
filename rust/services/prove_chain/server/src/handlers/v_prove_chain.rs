use serde::{Deserialize, Serialize};

use crate::error::Error;

#[derive(Deserialize, Serialize)]
pub struct Params {}

#[derive(Serialize)]
pub struct ChainProof;

pub async fn v_prove_chain(_params: Params) -> Result<ChainProof, Error> {
    Ok(ChainProof)
}

use alloy_primitives::BlockHash;
use serde::{Deserialize, Serialize};
use server_utils::parse_hash_field;

use crate::error::AppError;

#[derive(Deserialize, Serialize)]
pub struct Params {
    block_hashes: Vec<String>,
}

pub struct ValidatedParams {
    block_hashes: Vec<BlockHash>,
}

impl TryFrom<Params> for ValidatedParams {
    type Error = AppError;

    fn try_from(value: Params) -> Result<Self, Self::Error> {
        let block_hashes = value
            .block_hashes
            .into_iter()
            .map(|hash| parse_hash_field("block hashes", hash))
            .collect::<Result<_, _>>()?;

        Ok(Self { block_hashes })
    }
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

#[tokio::test]
async fn empty_block_hashes() {
    let empty_block_hashes = Params {
        block_hashes: vec![],
    };
    assert_eq!(
        v_prove_chain(empty_block_hashes).await.unwrap_err(),
        AppError::NoBlockHashes
    );
}

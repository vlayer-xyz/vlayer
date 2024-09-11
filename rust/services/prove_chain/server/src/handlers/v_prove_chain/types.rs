use alloy_primitives::BlockHash;
use server_utils::parse_hash_field;

use crate::error::AppError;

use super::Params;

pub struct ValidatedParams {
    pub block_hashes: Vec<BlockHash>,
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

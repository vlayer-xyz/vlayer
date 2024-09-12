use serde::{Deserialize, Serialize};

use crate::error::AppError;

#[derive(Deserialize, Serialize)]
pub struct Params {
    block_hashes: Vec<String>,
}

#[derive(Serialize, Debug, PartialEq)]
pub struct ChainProof {
    receipt: Vec<u8>,
}

pub async fn v_prove_chain(params: Params) -> Result<ChainProof, AppError> {
    if params.block_hashes.is_empty() {
        return Err(AppError::NoBlockHashes);
    };

    Ok(ChainProof { receipt: vec![] })
}

#[cfg(test)]
mod tests {
    use super::*;

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

    #[tokio::test]
    async fn success() -> Result<(), AppError> {
        let parent_block_hash =
            "0xb390d63aac03bbef75de888d16bd56b91c9291c2a7e38d36ac24731351522bd1".to_string(); // https://etherscan.io/block/19999999
        let block_hash =
            "0xd24fd73f794058a3807db926d8898c6481e902b7edb91ce0d479d6760f276183".to_string(); // https://etherscan.io/block/20000000
        let params = Params {
            block_hashes: vec![parent_block_hash, block_hash],
        };

        assert!(v_prove_chain(params).await.is_ok());

        Ok(())
    }
}

use std::collections::HashMap;

use prove_chain_host::{Host, HostConfig, HostOutput, ProofMode};
use risc0_zkvm::Receipt;
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

impl TryFrom<Receipt> for ChainProof {
    type Error = AppError;
    fn try_from(value: Receipt) -> Result<Self, Self::Error> {
        let receipt =
            bincode::serialize(&value).map_err(|err| AppError::Bincode(err.to_string()))?;
        Ok(ChainProof { receipt })
    }
}

pub async fn v_prove_chain(params: Params) -> Result<ChainProof, AppError> {
    if params.block_hashes.is_empty() {
        return Err(AppError::NoBlockHashes);
    };

    let config = HostConfig {
        rpc_urls: HashMap::new(),
        proof_mode: ProofMode::Fake,
    };
    let HostOutput { receipt } = Host::new(config)?.run()?;

    Ok(receipt.try_into()?)
}

#[cfg(test)]
mod tests {
    use super::*;
    use anyhow::Result;
    use risc0_zkvm::Receipt;

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
    async fn returns_valid_receipt() -> Result<()> {
        let parent_block_hash =
            "0xb390d63aac03bbef75de888d16bd56b91c9291c2a7e38d36ac24731351522bd1".to_string(); // https://etherscan.io/block/19999999
        let block_hash =
            "0xd24fd73f794058a3807db926d8898c6481e902b7edb91ce0d479d6760f276183".to_string(); // https://etherscan.io/block/20000000
        let params = Params {
            block_hashes: vec![parent_block_hash, block_hash],
        };

        let chain_proof = v_prove_chain(params).await?;
        let _: Receipt = bincode::deserialize(chain_proof.receipt.as_slice())?;

        Ok(())
    }
}

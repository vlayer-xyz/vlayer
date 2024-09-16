use std::sync::Arc;

use axum_jrpc::Value;
use mpt::MerkleTrie;
use serde::{Deserialize, Serialize};
use types::ValidatedParams;

use crate::{config::ServerConfig, error::AppError};

pub mod types;

#[derive(Deserialize, Serialize)]
pub struct Params {
    block_hashes: Vec<String>,
}

#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub struct ChainProof {
    merkle_trie: MerkleTrie,
}

impl ChainProof {
    pub fn to_json(&self) -> Value {
        serde_json::to_value(self).expect("ChainProof to json")
    }
}

pub async fn v_prove_chain(
    _config: Arc<ServerConfig>,
    merkle_trie: MerkleTrie,
    params: Params,
) -> Result<ChainProof, AppError> {
    let params: ValidatedParams = params.try_into()?;
    if params.block_hashes.is_empty() {
        return Err(AppError::NoBlockHashes);
    };

    Ok(ChainProof { merkle_trie })
}

#[cfg(test)]
mod tests {
    use super::*;
    use alloy_primitives::{fixed_bytes, FixedBytes};
    use anyhow::Result;
    use lazy_static::lazy_static;
    use server_utils::ProofMode;

    lazy_static! {
        static ref config: Arc<ServerConfig> = Arc::new(ServerConfig {
            proof_mode: ProofMode::Fake,
            ..Default::default()
        });
        static ref parent_block_hash: String = "0xb390d63aac03bbef75de888d16bd56b91c9291c2a7e38d36ac24731351522bd1".to_string(); // https://etherscan.io/block/19999999
        static ref child_block_hash: String = "0xd24fd73f794058a3807db926d8898c6481e902b7edb91ce0d479d6760f276183".to_string(); // https://etherscan.io/block/20000000
    }

    #[tokio::test]
    async fn empty_block_hashes() {
        let empty_block_hashes = Params {
            block_hashes: vec![],
        };
        let trie = MerkleTrie::default();
        assert_eq!(
            v_prove_chain(config.clone(), trie, empty_block_hashes)
                .await
                .unwrap_err(),
            AppError::NoBlockHashes
        );
    }

    fn verify_response(response: ChainProof, expected_root: FixedBytes<32>) {
        let ChainProof { merkle_trie } = response;
        let root = merkle_trie.hash_slow();
        assert_eq!(root, expected_root)
    }

    #[tokio::test]
    async fn two_consecutive_block_hashes() -> Result<()> {
        let params = Params {
            block_hashes: vec![parent_block_hash.clone(), child_block_hash.clone()],
        };
        let mut trie = MerkleTrie::default();

        for (idx, block_hash) in params.block_hashes.iter().enumerate() {
            let hex: FixedBytes<32> = block_hash.parse()?;
            trie.insert([idx as u8], hex)?;
        }

        let response = v_prove_chain(config.clone(), trie, params).await?;

        verify_response(
            response,
            fixed_bytes!("1f85ea9c12d6a78a33d70d4759fabf710fa67ba3a2d215348c779c6861c8c5ac"),
        );

        Ok(())
    }
}

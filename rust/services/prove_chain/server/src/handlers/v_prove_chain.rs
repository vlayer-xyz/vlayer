use std::sync::Arc;

use mpt::MerkleTrie;
use serde::{Deserialize, Serialize};

use crate::{config::ServerConfig, error::AppError};

#[derive(Deserialize, Serialize)]
pub struct Params {
    chain_id: u32,
    block_numbers: Vec<u32>,
}

#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub struct ChainProof {
    merkle_trie: MerkleTrie,
}

pub async fn v_prove_chain(
    _config: Arc<ServerConfig>,
    merkle_trie: MerkleTrie,
    params: Params,
) -> Result<ChainProof, AppError> {
    if params.block_numbers.is_empty() {
        return Err(AppError::NoBlockNumbers);
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
        static ref parent_block_hash: String = "0x88e96d4537bea4d9c05d12549907b32561d3bf31f45aae734cdc119f13406cb6".to_string(); // https://etherscan.io/block/1
        static ref child_block_hash: String = "0xb495a1d7e6663152ae92708da4843337b958146015a2802f4193a410044698c9".to_string(); // https://etherscan.io/block/2
    }

    #[tokio::test]
    async fn empty_block_hashes() {
        let empty_block_hashes = Params {
            chain_id: 1,
            block_numbers: vec![],
        };
        let trie = MerkleTrie::default();
        assert_eq!(
            v_prove_chain(config.clone(), trie, empty_block_hashes)
                .await
                .unwrap_err(),
            AppError::NoBlockNumbers
        );
    }

    #[tokio::test]
    async fn two_consecutive_block_hashes() -> Result<()> {
        let mut trie = MerkleTrie::default();
        let parent_hash: FixedBytes<32> = parent_block_hash.parse()?;
        trie.insert([1], parent_hash)?;
        let child_hash: FixedBytes<32> = child_block_hash.parse()?;
        trie.insert([2], child_hash)?;

        let params = Params {
            chain_id: 1,
            block_numbers: vec![1, 2],
        };

        let response = v_prove_chain(config.clone(), trie, params).await?;

        let ChainProof { proof, nodes } = response;
        let trie = MerkleTrie::from_rlp_nodes(nodes)?;
        let root = trie.hash_slow();

        assert_eq!(
            root,
            fixed_bytes!("94d2f2f7b7d20826dace8c875192670a01c64a20f0b2f19cfbfb942b1515af4d")
        );
        assert_eq!(trie.get([1]).unwrap(), parent_hash);
        assert_eq!(trie.get([2]).unwrap(), child_hash);

        Ok(())
    }
}

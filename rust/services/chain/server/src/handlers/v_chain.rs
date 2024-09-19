use std::sync::Arc;

use alloy_primitives::{BlockNumber, Bytes, ChainId};
use chain_host::{Host, HostConfig, HostOutput};
use mpt::MerkleTrie;
use serde::{Deserialize, Serialize};

use crate::{config::ServerConfig, error::AppError};

#[derive(Deserialize, Serialize, Clone, Debug)]
pub struct Params {
    chain_id: ChainId,
    block_numbers: Vec<BlockNumber>,
}

#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub struct ChainProof {
    proof: Bytes,
    nodes: Vec<Bytes>,
}

pub async fn v_chain(
    config: Arc<ServerConfig>,
    merkle_trie: MerkleTrie,
    params: Params,
) -> Result<ChainProof, AppError> {
    if params.block_numbers.is_empty() {
        return Err(AppError::NoBlockNumbers);
    };

    let host_config = HostConfig {
        rpc_urls: config.rpc_urls.clone(),
        proof_mode: config.proof_mode.into(),
    };
    let host = Host::new(&host_config);
    let HostOutput { receipt } = host.run(params.chain_id, &params.block_numbers, &merkle_trie)?;
    let proof =
        bincode::serialize(&receipt.inner).map_err(|err| AppError::Bincode(err.to_string()))?;

    Ok(ChainProof {
        proof: proof.into(),
        nodes: merkle_trie.to_rlp_nodes().map(Bytes::from).collect(),
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use lazy_static::lazy_static;
    use server_utils::ProofMode;

    lazy_static! {
        static ref config: Arc<ServerConfig> = Arc::new(ServerConfig {
            proof_mode: ProofMode::Fake,
            ..Default::default()
        });
    }

    #[tokio::test]
    async fn empty_block_hashes() {
        let empty_block_hashes = Params {
            chain_id: 1,
            block_numbers: vec![],
        };
        let trie = MerkleTrie::default();
        assert_eq!(
            v_chain(config.clone(), trie, empty_block_hashes).await.unwrap_err(),
            AppError::NoBlockNumbers
        );
    }

    mod two_consecutive_block_hashes {
        use super::*;
        use alloy_primitives::{fixed_bytes, FixedBytes};
        use anyhow::Result;
        use chain_host::RISC0_CHAIN_GUEST_ID;
        use risc0_zkp::verify::VerificationError;
        use risc0_zkvm::{InnerReceipt, Receipt};

        lazy_static! {
            static ref parent_hash: FixedBytes<32> = fixed_bytes!("88e96d4537bea4d9c05d12549907b32561d3bf31f45aae734cdc119f13406cb6"); // https://etherscan.io/block/1
            static ref child_hash: FixedBytes<32> = fixed_bytes!("b495a1d7e6663152ae92708da4843337b958146015a2802f4193a410044698c9"); // https://etherscan.io/block/2
            static ref db_trie: MerkleTrie =
                MerkleTrie::from_iter([([1], *parent_hash), ([2], *child_hash)]);
            static ref params: Params = Params {
                chain_id: 1,
                block_numbers: vec![1, 2],
            };
        }

        #[tokio::test]
        async fn trie_contains_proofs() -> Result<()> {
            let response = v_chain(config.clone(), db_trie.clone(), params.clone()).await?;

            let ChainProof { nodes, .. } = response;
            let trie = MerkleTrie::from_rlp_nodes(nodes)?;

            assert_eq!(
                trie.hash_slow(),
                fixed_bytes!("cdb081c8a4b30d52307c3bebbc49a8f1520c0f936a0802e8bbc4e04dff17dbaa")
            );
            assert_eq!(trie.get([1]).unwrap(), *parent_hash);
            assert_eq!(trie.get([2]).unwrap(), *child_hash);

            Ok(())
        }

        #[tokio::test]
        async fn proof_does_verify() -> Result<()> {
            let response = v_chain(config.clone(), db_trie.clone(), params.clone()).await?;

            let ChainProof { proof, nodes } = response;
            let trie = MerkleTrie::from_rlp_nodes(nodes)?;

            let inner_receipt: InnerReceipt = bincode::deserialize(&proof)?;
            let receipt = Receipt::new(inner_receipt, trie.hash_slow().to_vec());
            assert!(receipt.verify(RISC0_CHAIN_GUEST_ID).is_ok());

            Ok(())
        }

        #[tokio::test]
        async fn proof_does_not_verify_with_invalid_elf_id() -> Result<()> {
            let response = v_chain(config.clone(), db_trie.clone(), params.clone()).await?;

            let ChainProof { proof, nodes } = response;
            let trie = MerkleTrie::from_rlp_nodes(nodes)?;

            let inner_receipt: InnerReceipt = bincode::deserialize(&proof)?;
            let receipt = Receipt::new(inner_receipt, trie.hash_slow().to_vec());

            let wrong_guest_id = [0; 32];

            assert!(matches!(
                receipt.verify(wrong_guest_id).unwrap_err(),
                VerificationError::ClaimDigestMismatch { .. }
            ));

            Ok(())
        }
    }
}

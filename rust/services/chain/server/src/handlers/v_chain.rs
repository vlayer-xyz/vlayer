use std::sync::Arc;

use alloy_primitives::{BlockNumber, Bytes, ChainId};
use mpt::MerkleTrie;
use chain_host::{Host, HostConfig, HostOutput};
use serde::{Deserialize, Serialize};

use crate::{config::ServerConfig, error::AppError};

#[derive(Deserialize, Serialize)]
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
    let host = Host::new(host_config);
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
    use alloy_primitives::{fixed_bytes, FixedBytes};
    use anyhow::Result;
    use lazy_static::lazy_static;
    use prove_chain_host::RISC0_PROVE_CHAIN_GUEST_ID;
    use risc0_zkvm::{InnerReceipt, Receipt};
    use server_utils::ProofMode;

    lazy_static! {
        static ref config: Arc<ServerConfig> = Arc::new(ServerConfig {
            proof_mode: ProofMode::Fake,
            ..Default::default()
        });
        static ref parent_hash: FixedBytes<32> = fixed_bytes!("88e96d4537bea4d9c05d12549907b32561d3bf31f45aae734cdc119f13406cb6"); // https://etherscan.io/block/1
        static ref child_hash: FixedBytes<32> = fixed_bytes!("b495a1d7e6663152ae92708da4843337b958146015a2802f4193a410044698c9"); // https://etherscan.io/block/2
    }

    #[tokio::test]
    async fn empty_block_hashes() {
        let empty_block_hashes = Params {
            chain_id: 1,
            block_numbers: vec![],
        };
        let trie = MerkleTrie::default();
        assert_eq!(
            v_chain(config.clone(), trie, empty_block_hashes)
                .await
                .unwrap_err(),
            AppError::NoBlockNumbers
        );
    }

    #[tokio::test]
    async fn two_consecutive_block_hashes() -> Result<()> {
        let trie = MerkleTrie::from_iter([([1], *parent_hash), ([2], *child_hash)]);

        let params = Params {
            chain_id: 1,
            block_numbers: vec![1, 2],
        };

        let response = v_chain(config.clone(), trie, params).await?;

        let ChainProof { proof, nodes } = response;
        let trie = MerkleTrie::from_rlp_nodes(nodes)?;
        let root_hash = trie.hash_slow();

        assert_eq!(
            root_hash,
            fixed_bytes!("cdb081c8a4b30d52307c3bebbc49a8f1520c0f936a0802e8bbc4e04dff17dbaa")
        );
        assert_eq!(trie.get([1]).unwrap(), *parent_hash);
        assert_eq!(trie.get([2]).unwrap(), *child_hash);

        let inner_receipt: InnerReceipt = bincode::deserialize(&proof)?;
        let receipt = Receipt::new(inner_receipt, root_hash.to_vec());
        assert!(receipt.verify(RISC0_PROVE_CHAIN_GUEST_ID).is_ok());

        Ok(())
    }
}

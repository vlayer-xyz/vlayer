use std::sync::Arc;

use alloy_primitives::{BlockNumber, ChainId};
use chain_common::RpcChainProof;
use chain_db::ChainDb;
use parking_lot::RwLock;

use crate::error::AppError;

#[cfg(test)]
#[allow(clippy::unreadable_literal)]
const SOME_RISC0_CHAIN_GUEST_ID: [u32; 8] = [
    2663174293, 2024089015, 3465834372, 887420448, 2606376422, 1669533029, 1010997213, 2366700158,
];

#[allow(clippy::unused_async)]
pub async fn v_get_chain_proof(
    chain_db: Arc<RwLock<ChainDb>>,
    chain_id: ChainId,
    block_numbers: Vec<BlockNumber>,
) -> Result<RpcChainProof, AppError> {
    if block_numbers.is_empty() {
        return Err(AppError::NoBlockNumbers);
    };

    let chain_proof = chain_db.read().get_chain_proof(chain_id, block_numbers)?;
    Ok(chain_proof.into())
}

#[cfg(test)]
mod tests {
    use common::GuestElf;
    use lazy_static::lazy_static;

    use super::*;

    #[tokio::test]
    async fn empty_block_hashes() {
        let chain_id: ChainId = 1;
        let block_numbers: Vec<BlockNumber> = vec![];
        let chain_db = Arc::new(RwLock::new(ChainDb::in_memory([GuestElf::default().id])));
        assert_eq!(
            v_get_chain_proof(chain_db, chain_id, block_numbers)
                .await
                .unwrap_err(),
            AppError::NoBlockNumbers
        );
    }

    mod two_consecutive_block_hashes {
        use ::chain_db::{ChainInfo, ChainUpdate};
        use alloy_primitives::{FixedBytes, bytes, fixed_bytes};
        use anyhow::Result;
        use bytes::Bytes;
        use common::Hashable;
        use mpt::Sha2Trie as MerkleTrie;
        use risc0_zkp::verify::VerificationError;
        use risc0_zkvm::{InnerReceipt, Receipt};
        use u64_range::NonEmptyRange;

        use super::*;

        lazy_static! {
            static ref zk_proof: Bytes = bytes!("0200000000010000000000002a6a30624f8e369e9e52beb870e2320d78faa7eaa0cde307c60bf0de5dd9ed5c1c5d530ba0d64f6005ff14036b57ec50ef4120808c0b9ad19573a1514d22d9a02379217a08dba9fffb5ed72a65e726339b03ade78082baea80b2e5218b813786299b8c688cfdf073c8c6f09ad14fe3a5f2a3a227da31733066f5bfff0bb12bd122e617b68420b3bd6f918b4c004dfdb70a213dc6be373efbf3820528782349d80aed2dca06bff75c969bc653c9df91fc27f2d74f8e0cb7c02ca7e8546772dffb1e9dc9131c22eb3b87a5130b4c25ae16d672aef2079c8ac6ec60ea6a93e337d21f7b2b0d8c67ff442912610a03825be61cba767ca0e1c4bc3579f4cc171e474600000000000000000436200086f57219536951276fb0b8b74d7342c5c8cfe74b953d8e16b343364199f8524e0000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000100000000000000000000000000000000000000000000000000000000000000000000000000000001000000002000000000000000cdb081c8a4b30d52307c3bebbc49a8f1520c0f936a0802e8bbc4e04dff17dbaa000000000000000000000000310fe598e8e3e92fa805bc272d7f587898bb8b68c4d5d7938db884abaa76e15c").into();
            static ref parent_hash: FixedBytes<32> = fixed_bytes!("88e96d4537bea4d9c05d12549907b32561d3bf31f45aae734cdc119f13406cb6"); // https://etherscan.io/block/1
            static ref child_hash: FixedBytes<32> = fixed_bytes!("b495a1d7e6663152ae92708da4843337b958146015a2802f4193a410044698c9"); // https://etherscan.io/block/2
            static ref db_trie: MerkleTrie =
                MerkleTrie::from_iter([([1], *parent_hash), ([2], *child_hash)]);
            static ref chain_db: Arc<RwLock<ChainDb>> = {
                let db = Arc::new(RwLock::new(ChainDb::in_memory([GuestElf::default().id])));
                let range = NonEmptyRange::try_from_range(1..=2).unwrap();
                let chain_info = ChainInfo::new(range, db_trie.hash_slow(), zk_proof.clone());
                db.write().update_chain(1, ChainUpdate::new(chain_info, &*db_trie, [])).expect("update_chain failed");
                db
            };
            static ref chain_id: ChainId = 1;
            static ref block_numbers: Vec<BlockNumber> = vec![1, 2];
        }

        #[ignore = "MPT hashes changed because of RLP encoding fix"]
        #[tokio::test]
        async fn trie_contains_proofs() -> Result<()> {
            let response =
                v_get_chain_proof(chain_db.clone(), *chain_id, block_numbers.clone()).await?;

            let RpcChainProof { nodes, .. } = response;
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
        async fn zk_proof_read_from_db() -> Result<()> {
            let RpcChainProof { proof, .. } =
                v_get_chain_proof(chain_db.clone(), *chain_id, block_numbers.clone()).await?;
            assert_eq!(proof, *zk_proof);
            Ok(())
        }

        #[ignore = "MPT hashes changed because of RLP encoding fix"]
        #[tokio::test]
        async fn proof_does_verify() -> Result<()> {
            let response =
                v_get_chain_proof(chain_db.clone(), *chain_id, block_numbers.clone()).await?;

            let RpcChainProof { proof, nodes } = response;
            let trie = MerkleTrie::from_rlp_nodes(nodes)?;

            let inner_receipt: InnerReceipt = bincode::deserialize(&proof)?;
            let receipt = Receipt::new(inner_receipt, trie.hash_slow().to_vec());
            assert!(receipt.verify(SOME_RISC0_CHAIN_GUEST_ID).is_ok());

            Ok(())
        }

        #[tokio::test]
        async fn proof_does_not_verify_with_invalid_elf_id() -> Result<()> {
            let response =
                v_get_chain_proof(chain_db.clone(), *chain_id, block_numbers.clone()).await?;

            let RpcChainProof { proof, nodes } = response;
            let trie = MerkleTrie::from_rlp_nodes(nodes)?;

            let inner_receipt: InnerReceipt = bincode::deserialize(&proof)?;
            let receipt = Receipt::new(inner_receipt, trie.hash_slow().to_vec());

            let wrong_guest_id = [0; 32];

            assert_eq!(
                receipt.verify(wrong_guest_id).unwrap_err(),
                VerificationError::InvalidProof
            );

            Ok(())
        }
    }
}

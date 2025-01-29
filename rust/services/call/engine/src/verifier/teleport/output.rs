use alloy_eips::BlockNumHash;
use alloy_primitives::{keccak256, B256, U256};
use async_trait::async_trait;
use common::Hashable;
use serde::{Deserialize, Serialize};

/// The block reference for an L2 block.
///
/// See: <https://github.com/ethereum-optimism/optimism/blob/develop/op-service/eth/id.go#L33>
#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct L2BlockRef {
    /// The l1 block info.
    #[serde(flatten)]
    pub l1_block_info: BlockInfo,
    /// The origin on L1.
    #[serde(rename = "l1origin")]
    pub l1_origin: BlockNumHash,
    /// The sequence number.
    pub sequence_number: u64,
}

/// Block Header Info
#[derive(Serialize, Deserialize, Debug, Clone, Copy, Eq, Hash, PartialEq, Default)]
#[serde(rename_all = "camelCase")]
pub struct BlockInfo {
    /// The block hash
    pub hash: B256,
    /// The block number
    #[serde(with = "alloy_serde::quantity")]
    pub number: u64,
    /// The parent block hash
    pub parent_hash: B256,
    /// The block timestamp
    #[serde(with = "alloy_serde::quantity")]
    pub timestamp: u64,
}

/// The [`SyncStatus`][ss] of an Optimism Rollup Node.
///
/// [ss]: https://github.com/ethereum-optimism/optimism/blob/develop/op-service/eth/sync_status.go#L5
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize, Default)]
pub struct SyncStatus {
    /// The current L1 block.
    pub current_l1: BlockInfo,
    /// The current L1 finalized block.
    pub current_l1_finalized: BlockInfo,
    /// The L1 head block ref.
    pub head_l1: BlockInfo,
    /// The L1 safe head block ref.
    pub safe_l1: BlockInfo,
    /// The finalized L1 block ref.
    pub finalized_l1: BlockInfo,
    /// The unsafe L2 block ref.
    pub unsafe_l2: L2BlockRef,
    /// The safe L2 block ref.
    pub safe_l2: L2BlockRef,
    /// The finalized L2 block ref.
    pub finalized_l2: L2BlockRef,
    /// The pending safe L2 block ref.
    pub pending_safe_l2: L2BlockRef,
}

/// An [output response][or] for Optimism Rollup.
///
/// [or]: https://github.com/ethereum-optimism/optimism/blob/f20b92d3eb379355c876502c4f28e72a91ab902f/op-service/eth/output.go#L10-L17
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct OutputResponse {
    /// The output version.
    pub version: B256,
    /// The output root hash.
    pub output_root: B256,
    /// A reference to the L2 block.
    pub block_ref: L2BlockRef,
    /// The withdrawal storage root.
    pub withdrawal_storage_root: B256,
    /// The state root.
    pub state_root: B256,
    /// The status of the node sync.
    pub sync_status: SyncStatus,
}

#[async_trait]
pub trait OpRpcClient: Send + Sync {
    async fn get_output_at_block(&self, block_number: U256) -> OutputResponse;
}

impl Hashable for OutputResponse {
    fn hash_slow(&self) -> B256 {
        let payload: Vec<u8> = [
            self.version.to_vec(),
            self.state_root.to_vec(),
            self.withdrawal_storage_root.to_vec(),
            self.sync_status.finalized_l2.l1_block_info.hash.to_vec(),
        ]
        .concat();

        keccak256(payload)
    }
}

#[cfg(test)]
mod hash_slow {
    use alloy_primitives::hex;
    use lazy_static::lazy_static;

    use super::*;

    lazy_static! {
        static ref STATE_ROOT: B256 =
            B256::from(hex!("b96b23e8db3147cf46b80eda0b97e6612cbdcec43128d5bd81a8360093cfcf17"));
        static ref WITHDRAWAL_STORAGE_ROOT: B256 =
            B256::from(hex!("1e346b4b9774c44851b6e75760e09da0495f0b9124282e0f652df80d9a876b44"));
        static ref FINALIZED_L2_HASH: B256 =
            B256::from(hex!("4cd86d480704aef6106fcd200a26f2d6e6025f1032dd9b6ae09af85198973cd9"));
        static ref OUTPUT: OutputResponse = OutputResponse {
            state_root: *STATE_ROOT,
            withdrawal_storage_root: *WITHDRAWAL_STORAGE_ROOT,
            sync_status: SyncStatus {
                finalized_l2: L2BlockRef {
                    l1_block_info: BlockInfo {
                        hash: *FINALIZED_L2_HASH,
                        ..Default::default()
                    },
                    ..Default::default()
                },
                ..Default::default()
            },
            ..Default::default()
        };
    }

    #[test]
    fn hash_slow_test() {
        let output = OUTPUT.clone();

        let hash = output.hash_slow();

        let expected_hash =
            B256::from(hex!("39e47bbb42c1043f6f05950f4af9dd673a521c31001cb87e47c2642040580f54"));
        assert_eq!(hash, expected_hash);
    }
}

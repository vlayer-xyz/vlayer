use alloy_eips::BlockNumHash;
use alloy_primitives::B256;
use serde::{Deserialize, Serialize};

/// The block reference for an L2 block.
///
/// See: <https://github.com/ethereum-optimism/optimism/blob/develop/op-service/eth/id.go#L33>
#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct L2BlockRef {
    /// The l1 block info.
    #[serde(flatten)]
    pub l2_block_info: BlockInfo,
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

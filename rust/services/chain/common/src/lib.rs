use alloy_primitives::{BlockNumber, ChainId};
use common::Method;
use derive_new::new;
use serde::{Deserialize, Serialize};

#[cfg(feature = "risc0")]
mod risc0;

#[cfg(feature = "risc0")]
pub use risc0::*;

#[cfg(feature = "sp1")]
mod sp1;

#[cfg(feature = "sp1")]
pub use sp1::*;

pub mod verifier;

#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize, Deserialize, new)]
pub struct GetSyncStatus {
    pub chain_id: ChainId,
}

impl Method for GetSyncStatus {
    const METHOD_NAME: &str = "v_getSyncStatus";
}

#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize, Deserialize, new)]
pub struct SyncStatus {
    pub first_block: BlockNumber,
    pub last_block: BlockNumber,
}

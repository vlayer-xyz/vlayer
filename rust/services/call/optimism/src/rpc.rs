pub mod client;

use alloy_primitives::{hex, B256, U256};
use async_trait::async_trait;
use lazy_static::lazy_static;

use crate::types::{BlockInfo, L2BlockRef, OutputResponse};

#[async_trait]
pub trait Client: Send + Sync {
    async fn get_output_at_block(&self, block_number: U256) -> OutputResponse;
}

pub struct DummyClient;

lazy_static! {
    static ref STATE_ROOT: B256 =
        B256::from(hex!("25d65fff68c2248f9b0c0b04d2ce9749dbdb088bd0fe16962476f18794373e5f"));
    static ref WITHDRAWAL_STORAGE_ROOT: B256 =
        B256::from(hex!("56e81f171bcc55a6ff8345e692c0f86e5b48e01b996cadc001622fb5e363b421"));
    static ref FINALIZED_L2_HASH: B256 =
        B256::from(hex!("f8714d13fc9772dc0230587b17c9458b39b1a94815b4bfebd0b0c8e55a6e2aab"));
    static ref OUTPUT: OutputResponse = OutputResponse {
        block_ref: L2BlockRef {
            l2_block_info: BlockInfo {
                hash: *FINALIZED_L2_HASH,
                number: 3,
                ..Default::default()
            },
            ..Default::default()
        },
        state_root: *STATE_ROOT,
        withdrawal_storage_root: *WITHDRAWAL_STORAGE_ROOT,
        ..Default::default()
    };
}

#[async_trait]
impl Client for DummyClient {
    async fn get_output_at_block(&self, _block_number: U256) -> OutputResponse {
        OUTPUT.clone()
    }
}

use alloy_primitives::{hex, ChainId, B256};
use derive_new::new;
use lazy_static::lazy_static;
use revm::primitives::HashMap;

use crate::{
    client::{mock, FactoryError, IFactory},
    types::{BlockInfo, L2BlockRef, OutputResponse},
    IClient,
};

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

#[derive(Debug, Clone, new, Default)]
pub struct Factory {
    sequencer_outputs: HashMap<ChainId, OutputResponse>,
}

impl Factory {
    /// Used in tests for convenience.
    pub fn from_single_sequencer_output(
        chain_id: ChainId,
        sequencer_output: OutputResponse,
    ) -> Self {
        Self {
            sequencer_outputs: [(chain_id, sequencer_output)].into_iter().collect(),
        }
    }
}

impl IFactory for Factory {
    fn create(&self, chain_id: ChainId) -> Result<Box<dyn IClient>, FactoryError> {
        let sequencer_output = self
            .sequencer_outputs
            .get(&chain_id)
            .ok_or(FactoryError::NoDataForChain(chain_id))?;

        let client = mock::Client::new(sequencer_output.clone());
        Ok(Box::new(client))
    }
}

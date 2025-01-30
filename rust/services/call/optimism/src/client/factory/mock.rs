use alloy_primitives::ChainId;
use derive_new::new;
use revm::primitives::HashMap;

use crate::{
    client::{mock, FactoryError, IFactory},
    types::OutputResponse,
    IClient,
};

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

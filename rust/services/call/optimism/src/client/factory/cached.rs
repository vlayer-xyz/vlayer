use std::sync::Arc;

use alloy_primitives::ChainId;
use derive_new::new;
use revm::primitives::HashMap;
use thiserror::Error;

use crate::{
    client::{cached, FactoryError, IFactory},
    types::OutputResponse,
    IClient,
};

#[derive(Debug, Error, PartialEq, Eq)]
pub enum Error {
    #[error("No Data for chain {0}")]
    NoDataForChain(ChainId),
}

pub type OpOutputCache = HashMap<ChainId, OutputResponse>;

#[derive(Debug, Clone, new, Default)]
pub struct Factory {
    cache: HashMap<ChainId, OutputResponse>,
}

impl Factory {
    /// Used in tests for convenience.
    pub fn from_single_sequencer_output(
        chain_id: ChainId,
        sequencer_output: OutputResponse,
    ) -> Self {
        Self {
            cache: [(chain_id, sequencer_output)].into_iter().collect(),
        }
    }
}

impl IFactory for Factory {
    fn create(&self, chain_id: ChainId) -> Result<Arc<dyn IClient>, FactoryError> {
        let sequencer_output = self
            .cache
            .get(&chain_id)
            .ok_or(Error::NoDataForChain(chain_id))?;

        let client = cached::Client::new(sequencer_output.clone());
        Ok(Arc::new(client))
    }
}

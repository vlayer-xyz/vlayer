use std::collections::HashMap;

use alloy_primitives::ChainId;
use derive_new::new;
use thiserror::Error;

use crate::{
    IClient,
    client::{FactoryError, IFactory, cached},
    types::SequencerOutput,
};

#[derive(Debug, Error, PartialEq, Eq)]
pub enum Error {
    #[error("No Data for chain {0}")]
    NoDataForChain(ChainId),
}

pub type OpOutputCache = HashMap<ChainId, SequencerOutput>;

#[derive(Debug, Clone, new, Default)]
pub struct Factory {
    cache: HashMap<ChainId, SequencerOutput>,
}

impl Factory {
    /// Used in tests for convenience.
    pub fn from_single_sequencer_output(
        chain_id: ChainId,
        sequencer_output: SequencerOutput,
    ) -> Self {
        Self {
            cache: [(chain_id, sequencer_output)].into_iter().collect(),
        }
    }
}

impl IFactory for Factory {
    fn create(&self, chain_id: ChainId) -> Result<Box<dyn IClient>, FactoryError> {
        let sequencer_output = self
            .cache
            .get(&chain_id)
            .ok_or(Error::NoDataForChain(chain_id))?;

        let client = cached::Client::new(sequencer_output.clone());
        Ok(Box::new(client))
    }
}

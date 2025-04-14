use alloy_primitives::BlockNumber;
use async_trait::async_trait;
use derive_new::new;
use thiserror::Error;

use crate::{ClientError, IClient, types::SequencerOutput};

#[derive(Debug, Error, PartialEq, Eq)]
#[non_exhaustive]
pub enum Error {
    #[error("Requested block {requested} but client has only data for block {present}")]
    BlockNumberMismatch {
        requested: BlockNumber,
        present: BlockNumber,
    },
}

#[derive(Clone, Debug, new)]
pub struct Client {
    cache: SequencerOutput,
}

#[async_trait]
impl IClient for Client {
    async fn get_output_at_block(&self, block_number: u64) -> Result<SequencerOutput, ClientError> {
        let l2_block = self.cache.l2_block;
        if block_number != l2_block.number {
            return Err(Error::BlockNumberMismatch {
                requested: block_number,
                present: l2_block.number,
            }
            .into());
        }
        Ok(self.cache.clone())
    }
}

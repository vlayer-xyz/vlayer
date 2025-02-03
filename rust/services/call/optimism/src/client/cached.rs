use alloy_primitives::BlockNumber;
use async_trait::async_trait;
use derive_new::new;
use thiserror::Error;

use crate::{types::OutputResponse, ClientError, IClient};

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
    cache: OutputResponse,
}

#[async_trait]
impl IClient for Client {
    async fn get_output_at_block(&self, block_number: u64) -> Result<OutputResponse, ClientError> {
        let l2_block_info = self.cache.block_ref.l2_block_info;
        if block_number != l2_block_info.number {
            return Err(Error::BlockNumberMismatch {
                requested: block_number,
                present: l2_block_info.number,
            }
            .into());
        }
        Ok(self.cache.clone())
    }
}

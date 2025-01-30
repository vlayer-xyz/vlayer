use derive_new::new;

use crate::{types::OutputResponse, ClientError, IClient};

#[derive(new)]
pub struct Client {
    cache: OutputResponse,
}

#[async_trait::async_trait]
impl IClient for Client {
    async fn get_output_at_block(&self, block_number: u64) -> Result<OutputResponse, ClientError> {
        let l2_block_info = self.cache.block_ref.l2_block_info;
        if block_number != l2_block_info.number {
            return Err(ClientError::BlockNumberMismatch {
                requested: block_number,
                present: l2_block_info.number,
            });
        }
        Ok(self.cache.clone())
    }
}

use std::sync::{Arc, RwLock};

use alloy_primitives::BlockNumber;

use super::cached;
use crate::{types::OutputResponse, ClientError, IClient};

pub struct Client {
    inner: Arc<dyn IClient>,
    cache: Arc<RwLock<OutputResponse>>,
}

impl Client {
    pub fn new(inner: Box<dyn IClient>) -> Self {
        Self {
            inner: inner.into(),
            cache: Arc::new(RwLock::new(Default::default())),
        }
    }

    fn into_cache(self) -> OutputResponse {
        let cache =
            Arc::try_unwrap(self.cache).expect("Trying to access cache while it's still in use");
        cache.into_inner().expect("poisoned lock")
    }

    #[allow(unused)]
    pub fn into_cached_client(self) -> cached::Client {
        cached::Client::new(self.into_cache())
    }
}

#[async_trait::async_trait]
impl IClient for Client {
    async fn get_output_at_block(
        &self,
        block_number: BlockNumber,
    ) -> Result<OutputResponse, ClientError> {
        let output = self.inner.get_output_at_block(block_number).await?;
        self.cache
            .write()
            .expect("poisoned lock")
            .clone_from(&output);
        Ok(output)
    }
}

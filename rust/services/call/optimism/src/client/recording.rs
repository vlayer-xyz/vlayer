use std::sync::{Arc, RwLock};

use alloy_primitives::BlockNumber;

use crate::{ClientError, IClient, types::SequencerOutput};

#[derive(Clone)]
pub struct Client {
    inner: Arc<dyn IClient>,
    cache: Arc<RwLock<SequencerOutput>>,
}

impl Client {
    pub fn new(inner: impl IClient + 'static) -> Self {
        Self {
            inner: Arc::new(inner),
            cache: Arc::new(RwLock::new(Default::default())),
        }
    }

    #[allow(clippy::expect_used)]
    pub fn into_cache(self) -> SequencerOutput {
        let cache =
            Arc::try_unwrap(self.cache).expect("Trying to access cache while it's still in use");
        cache.into_inner().expect("poisoned lock")
    }
}

#[async_trait::async_trait]
#[allow(clippy::expect_used)]
impl IClient for Client {
    async fn get_output_at_block(
        &self,
        block_number: BlockNumber,
    ) -> Result<SequencerOutput, ClientError> {
        let output = self.inner.get_output_at_block(block_number).await?;
        self.cache
            .write()
            .expect("poisoned lock")
            .clone_from(&output);
        Ok(output)
    }
}

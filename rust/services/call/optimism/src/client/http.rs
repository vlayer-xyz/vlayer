use alloy_eips::BlockNumberOrTag;
use async_trait::async_trait;
use jsonrpsee::{core::RpcResult, http_client::HttpClient, proc_macros::rpc};

use crate::{types::OutputResponse, ClientError, IClient};

#[rpc(server, client, namespace = "optimism")]
pub trait RollupNode {
    /// Get the output root at a specific block.
    #[method(name = "outputAtBlock")]
    async fn op_output_at_block(&self, block_number: BlockNumberOrTag)
        -> RpcResult<OutputResponse>;
}

pub struct Client {
    client: HttpClient,
}

impl Client {
    pub const fn new(client: HttpClient) -> Self {
        Self { client }
    }
}

#[async_trait]
impl IClient for Client {
    async fn get_output_at_block(&self, block_number: u64) -> Result<OutputResponse, ClientError> {
        RollupNodeClient::op_output_at_block(&self.client, block_number.into())
            .await
            .map_err(|err| ClientError::JsonRPSee(err.to_string()))
    }
}

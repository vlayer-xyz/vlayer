use alloy_eips::BlockNumberOrTag;
use async_trait::async_trait;
use derivative::Derivative;
use jsonrpsee::{core::RpcResult, http_client::HttpClient, proc_macros::rpc};
use thiserror::Error;

use crate::{
    ClientError, IClient,
    types::{SequencerOutput, rpc::OutputResponse},
};

#[rpc(server, client, namespace = "optimism")]
pub trait RollupNode {
    /// Get the output root at a specific block.
    #[method(name = "outputAtBlock")]
    async fn op_output_at_block(&self, block_number: BlockNumberOrTag)
    -> RpcResult<OutputResponse>;
}

#[derive(Debug, Error, Derivative)]
#[derivative(PartialEq, Eq)]
pub enum Error {
    #[error("JsonRPSee error: {0}")]
    JsonRPSee(
        #[from]
        #[derivative(PartialEq = "ignore")]
        jsonrpsee::core::ClientError,
    ),
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
    async fn get_output_at_block(&self, block_number: u64) -> Result<SequencerOutput, ClientError> {
        let output = RollupNodeClient::op_output_at_block(&self.client, block_number.into())
            .await
            .map_err(Error::JsonRPSee)?;
        Ok(output.into())
    }
}

use std::sync::Arc;

use alloy_primitives::{BlockNumber, ChainId};
use async_trait::async_trait;
use chain_common::{RpcChainProof, SyncStatus};
use chain_db::ChainDb;
use jsonrpsee::proc_macros::rpc;
use parking_lot::RwLock;

use crate::error::AppError;

pub mod chain_proof;
pub mod status;

#[derive(Clone)]
pub struct State(Arc<RwLock<ChainDb>>);

impl State {
    pub fn new(chain_db: ChainDb) -> Self {
        Self(Arc::new(RwLock::new(chain_db)))
    }
}

#[rpc(server)]
#[async_trait]
pub trait Rpc {
    #[method(name = "v_getChainProof")]
    async fn v_get_chain_proof(
        &self,
        chain_id: ChainId,
        block_numbers: Vec<BlockNumber>,
    ) -> Result<RpcChainProof, AppError>;

    #[method(name = "v_getSyncStatus")]
    async fn v_sync_status(&self, chain_id: ChainId) -> Result<SyncStatus, AppError>;
}

#[async_trait]
impl RpcServer for State {
    async fn v_get_chain_proof(
        &self,
        chain_id: ChainId,
        block_numbers: Vec<BlockNumber>,
    ) -> Result<RpcChainProof, AppError> {
        chain_proof::v_get_chain_proof(self.0.clone(), chain_id, block_numbers).await
    }

    async fn v_sync_status(&self, chain_id: ChainId) -> Result<SyncStatus, AppError> {
        status::v_sync_status(self.0.clone(), chain_id).await
    }
}

use alloy_primitives::ChainId;
use async_trait::async_trait;
use chain_common::{ChainProof, SyncStatus, fake_proof_result};
use derive_new::new;
use provider::{BlockNumber, BlockingProviderExt, CachedMultiProvider};
use risc0_zkp::core::digest::Digest;

use crate::{Client, Error};

/// `Client` which doesn't connect to any server at all, but generates fake proofs on demand
#[derive(new)]
pub struct FakeClient {
    providers: CachedMultiProvider,
    guest_id: Digest,
}

#[async_trait]
impl Client for FakeClient {
    async fn get_chain_proof(
        &self,
        chain_id: ChainId,
        block_numbers: Vec<BlockNumber>,
    ) -> Result<ChainProof, Error> {
        let provider = self
            .providers
            .get(chain_id)
            .map_err(|_| Error::UnsupportedChain(chain_id))?;
        let block_tags = block_numbers.into_iter().map(Into::into).collect();
        let block_headers = provider
            .get_block_headers(block_tags)
            .await
            .map_err(Error::other)?;
        let rpc_chain_proof = fake_proof_result(self.guest_id, block_headers);
        Ok(rpc_chain_proof.try_into()?)
    }

    async fn get_sync_status(&self, chain_id: ChainId) -> Result<SyncStatus, Error> {
        let last_block = self
            .providers
            .get(chain_id)
            .map_err(|_| Error::UnsupportedChain(chain_id))?
            .get_latest_block_number()
            .map_err(Error::other)?;
        Ok(SyncStatus {
            first_block: 0,
            last_block,
        })
    }
}

/// `Client` which returns a const sync status
#[derive(new)]
pub struct PartiallySyncedClient {
    status: SyncStatus,
}

#[async_trait]
impl Client for PartiallySyncedClient {
    async fn get_chain_proof(&self, _: ChainId, _: Vec<BlockNumber>) -> Result<ChainProof, Error> {
        unimplemented!()
    }

    async fn get_sync_status(&self, _: ChainId) -> Result<SyncStatus, Error> {
        Ok(self.status.clone())
    }
}

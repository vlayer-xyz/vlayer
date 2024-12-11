use alloy_primitives::ChainId;
use axum_jrpc::Value;
use block_header::EvmBlockHeader;
use block_trie::BlockTrie;
use bytes::Bytes;
use chain_common::{GetChainProof, GetSyncStatus, RpcChainProof};
use common::{GuestElf, Hashable, Method};
use lazy_static::lazy_static;
use risc0_zkvm::{serde::to_vec, FakeReceipt, Receipt, ReceiptClaim};
use serde_json::json;
use server_utils::rpc::mock::{MockBuilder, Server as RpcServerMock};

lazy_static! {
    pub static ref EMPTY_PROOF_RESPONSE: Value =
        serde_json::to_value(RpcChainProof::default()).unwrap();
}

const GUEST_ELF: GuestElf = GuestElf::default();

fn fake_proof_result(block_header: Box<dyn EvmBlockHeader>) -> Value {
    let block_trie = BlockTrie::init(block_header).unwrap();
    let root_hash = block_trie.hash_slow();
    let proof_output = to_vec(&(root_hash, GUEST_ELF.id)).unwrap();
    let journal: Vec<u8> = bytemuck::cast_slice(&proof_output).into();
    let inner: FakeReceipt<ReceiptClaim> =
        FakeReceipt::<ReceiptClaim>::new(ReceiptClaim::ok(GUEST_ELF.id, journal.clone()));
    let receipt = Receipt::new(risc0_zkvm::InnerReceipt::Fake(inner), journal);
    let encoded_proof = bincode::serialize(&receipt).unwrap().into();
    let nodes: Vec<Bytes> = block_trie.into_iter().collect();

    serde_json::to_value(RpcChainProof::new(encoded_proof, nodes)).unwrap()
}

pub struct ChainProofServerMock {
    mock_server: RpcServerMock,
}

impl ChainProofServerMock {
    pub async fn start() -> Self {
        let mock_server = RpcServerMock::start().await;
        ChainProofServerMock { mock_server }
    }

    #[must_use]
    pub fn mock_chain_proof(&mut self) -> MockBuilder<'_> {
        self.mock_server.mock_method(GetChainProof::METHOD_NAME)
    }

    #[must_use]
    pub fn mock_sync_status(&mut self) -> MockBuilder<'_> {
        self.mock_server.mock_method(GetSyncStatus::METHOD_NAME)
    }

    pub async fn mock_single_block(
        &mut self,
        chain_id: ChainId,
        block_header: Box<dyn EvmBlockHeader>,
    ) {
        let block_number = block_header.number();
        self.mock_sync_status()
            .with_params(json!({"chain_id": chain_id}), true)
            .with_result(json!({
                "first_block": block_number,
                "last_block": block_number
            }))
            .with_expected_calls(0)
            .add()
            .await;
        self.mock_chain_proof()
            .with_params(
                json!({
                    "chain_id": chain_id,
                    "block_numbers": [block_number]
                }),
                true,
            )
            .with_result(fake_proof_result(block_header))
            .with_expected_calls(0)
            .add()
            .await;
    }

    pub fn url(&self) -> String {
        self.mock_server.url()
    }

    pub fn assert(&self) {
        self.mock_server.assert();
    }
}

#[cfg(test)]
mod tests {
    use alloy_primitives::B256;
    use block_header::test_utils::mock_block_header;
    use chain_client::{Client, RpcClient};
    use chain_common::SyncStatus;

    use super::*;

    #[tokio::test(flavor = "multi_thread")]
    async fn no_methods() {
        let chain_server = ChainProofServerMock::start().await;
        let client = RpcClient::new(chain_server.url());

        let chain_proof_result = client.get_chain_proof(1, vec![1]).await;
        assert!(matches!(
            chain_proof_result.unwrap_err(),
            chain_client::Error::Rpc(server_utils::rpc::Error::Http(_))
        ));

        let sync_status_result = client.get_sync_status(1).await;
        assert!(matches!(
            sync_status_result.unwrap_err(),
            chain_client::Error::Rpc(server_utils::rpc::Error::Http(_))
        ))
    }

    #[tokio::test(flavor = "multi_thread")]
    async fn chain_proof() {
        let mut chain_server = ChainProofServerMock::start().await;
        let rpc_chain_proof = RpcChainProof::new(Default::default(), vec![]);
        chain_server
            .mock_chain_proof()
            .with_params(GetChainProof::new(1, vec![1]), true)
            .with_result(rpc_chain_proof.clone())
            .with_expected_calls(1)
            .add()
            .await;
        let client = RpcClient::new(chain_server.url());

        let chain_proof_result = client.get_chain_proof(1, vec![1]).await;
        let expected_result = rpc_chain_proof.try_into().unwrap();
        assert_eq!(chain_proof_result.unwrap(), expected_result);
        chain_server.assert();
    }

    #[tokio::test(flavor = "multi_thread")]
    async fn sync_status() {
        let mut chain_server = ChainProofServerMock::start().await;
        let sync_status = SyncStatus::new(1, 1);
        chain_server
            .mock_sync_status()
            .with_params(GetSyncStatus::new(1), true)
            .with_result(sync_status.clone())
            .with_expected_calls(1)
            .add()
            .await;
        let client = RpcClient::new(chain_server.url());

        let sync_status_result = client.get_sync_status(1).await;
        assert_eq!(sync_status_result.unwrap(), sync_status);
        chain_server.assert();
    }

    #[tokio::test(flavor = "multi_thread")]
    async fn single_block() {
        let mut chain_server = ChainProofServerMock::start().await;
        let block_header = mock_block_header(1, B256::default());
        chain_server.mock_single_block(1, block_header).await;
        let client = RpcClient::new(chain_server.url());

        let chain_proof_result = client.get_chain_proof(1, vec![1]).await;
        let block_trie = chain_proof_result.unwrap().block_trie;
        assert!(block_trie.get(1).is_some());

        let sync_status_result = client.get_sync_status(1).await;
        assert_eq!(sync_status_result.unwrap(), SyncStatus::new(1, 1));
    }
}

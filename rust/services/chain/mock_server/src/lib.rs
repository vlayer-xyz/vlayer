use alloy_primitives::ChainId;
use axum_jrpc::Value;
use block_header::EvmBlockHeader;
use block_trie::KeccakBlockTrie as BlockTrie;
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

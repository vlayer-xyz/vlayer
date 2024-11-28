use axum_jrpc::Value;
use block_header::EvmBlockHeader;
use block_trie::BlockTrie;
use bytes::Bytes;
use chain_common::RpcChainProof;
use chain_guest_wrapper::GUEST_ELF;
use common::Hashable;
use lazy_static::lazy_static;
use risc0_zkvm::{serde::to_vec, FakeReceipt, Receipt, ReceiptClaim};
use serde::Serialize;
use server_utils::RpcServerMock;

lazy_static! {
    pub static ref EMPTY_PROOF_RESPONSE: Value =
        serde_json::to_value(RpcChainProof::default()).unwrap();
}

pub fn fake_proof_result(block_header: Box<dyn EvmBlockHeader>) -> Value {
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
    pub async fn start(
        params: impl Serialize,
        result: impl Serialize,
        expected_calls: usize,
    ) -> Self {
        let mock_server =
            RpcServerMock::start("v_chain", true, params, result, expected_calls).await;

        ChainProofServerMock { mock_server }
    }

    pub fn url(&self) -> String {
        self.mock_server.url()
    }

    pub fn assert(&self) {
        self.mock_server.assert();
    }
}

use std::sync::LazyLock;

use axum_jrpc::Value;
use serde::Serialize;
use serde_json::json;
use server_utils::RpcServerMock;

pub static EMPTY_PROOF_RESPONSE: LazyLock<Value> = LazyLock::new(|| {
    json!({
        "proof": "",
        "nodes": []
    })
});

pub struct ChainProofServerMock {
    mock_server: RpcServerMock,
}

impl ChainProofServerMock {
    pub async fn start(params: impl Serialize, result: impl Serialize) -> Self {
        let mock_server = RpcServerMock::start("v_chain", true, params, result).await;

        ChainProofServerMock { mock_server }
    }

    pub fn url(&self) -> String {
        self.mock_server.url()
    }

    pub fn assert(&self) {
        self.mock_server.assert();
    }
}

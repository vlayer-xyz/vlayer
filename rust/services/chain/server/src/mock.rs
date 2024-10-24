use axum_jrpc::Value;
use lazy_static::lazy_static;
use serde::Serialize;
use serde_json::json;
use server_utils::RpcServerMock;

lazy_static! {
    pub static ref EMPTY_PROOF_RESPONSE: Value = json!({
        "proof": "",
        "nodes": []
    });
}

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

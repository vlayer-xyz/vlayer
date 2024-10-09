use httpmock::{Mock, MockServer};
use serde_json::json;

use crate::handlers::v_chain::ChainProof;

pub struct ChainProofMock {
    pub mock_server: MockServer,
    pub url: String,
}

impl ChainProofMock {
    pub fn new() -> Self {
        let mock_server = MockServer::start();
        let chain_proof_url = mock_server.url("/");

        let chain_proof = ChainProof::default();
        let chain_proof_json = serde_json::to_value(&chain_proof).unwrap();

        let mock = mock_server.mock(|when, then| {
            when.method("POST")
                .path("/")
                .header("Content-Type", "application/json")
                .json_body_partial(
                    serde_json::to_string(&json!({
                        "method": "v_chain"
                    }))
                    .unwrap(),
                );

            then.status(200)
                .header("Content-Type", "application/json")
                .body(
                    serde_json::to_string(&json!({
                        "jsonrpc": "2.0",
                        "result": chain_proof_json,
                        "id": 1
                    }))
                    .unwrap(),
                );
        });

        ChainProofMock {
            mock_server,
            url: chain_proof_url,
        }
    }
}

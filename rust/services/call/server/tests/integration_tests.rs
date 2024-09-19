#![recursion_limit = "512"]

use axum::http::StatusCode;
use serde_json::json;

mod test_helpers;

use server_utils::{body_to_json, body_to_string};
use test_helpers::TestHelper;

mod server_tests {
    use super::*;

    #[tokio::test]
    async fn http_not_found() {
        let helper = TestHelper::create().await;
        let response = helper.post("/non_existent_http_path", &()).await;

        assert_eq!(StatusCode::NOT_FOUND, response.status());
        assert!(body_to_string(response.into_body()).await.is_empty());
    }

    #[tokio::test]
    async fn json_rpc_not_found() {
        let helper = TestHelper::create().await;

        let req = json!({
            "method": "non_existent_json_rpc_method",
            "params": [],
            "id": 1,
            "jsonrpc": "2.0",
        });
        let response = helper.post("/", &req).await;

        assert_eq!(StatusCode::OK, response.status());
        assert_eq!(
            json!({
                "jsonrpc": "2.0",
                "id": 1,
                "error": {
                    "code": -32601,
                    "message": "Method `non_existent_json_rpc_method` not found",
                    "data": null
                }
            }),
            body_to_json(response.into_body()).await
        );
    }

    mod v_call {
        use crate::test_helpers::WebProof;
        use assert_json_diff::assert_json_include;
        use server_utils::function_selector;
        use web_proof::fixtures::{load_web_proof_fixture, NOTARY_PUB_KEY_PEM_EXAMPLE};

        use super::*;
        use ethers::{abi::AbiEncode, types::Uint8, types::U256};

        #[tokio::test]
        async fn field_validation_error() {
            let helper = TestHelper::create().await;

            let req = json!({
                "method": "v_call",
                "params": [
                    {
                        "to": "I am not a valid address!",
                        "data": helper.contract.sum(U256::from(1), U256::from(2)).calldata().unwrap()
                    },
                    {
                    }
                    ],
                "id": 1,
                "jsonrpc": "2.0",
            });
            let response = helper.post("/", &req).await;

            assert_eq!(StatusCode::OK, response.status());
            assert_eq!(
                json!({
                    "jsonrpc": "2.0",
                    "id": 1,
                    "error": {
                        "code": -32602,
                        "message": "Invalid field: `to` Odd number of digits `I am not a valid address!`",
                        "data": null
                    }
                }),
                body_to_json(response.into_body()).await
            );
        }

        #[tokio::test]
        async fn success_simple_contract_call() {
            let helper = TestHelper::create().await;
            let call_data = helper.contract.sum(U256::from(1), U256::from(2)).calldata().unwrap();

            let req = json!({
                "method": "v_call",
                "params": [
                    {
                        "to": helper.contract.address(),
                        "data": call_data
                    },
                    {
                        "chain_id": 11155111
                    }
                    ],
                "id": 1,
                "jsonrpc": "2.0",
            });
            let response = helper.post("/", &req).await;

            assert_eq!(StatusCode::OK, response.status());
            assert_json_include!(
                expected: json!({
                        "jsonrpc": "2.0",
                        "id": 1,
                        "result": {
                            "evm_call_result": U256::from(3).encode_hex(),
                            "proof": {
                                "length": 160,
                                "seal": {
                                    "verifierSelector": "0xdeafbeef",
                                    "mode": 1,
                                },
                                "commitment": {
                                    "functionSelector": function_selector(&call_data),
                                    "proverContractAddress": helper.contract.address(),
                                }
                            },
                        }
                    }),
                actual: body_to_json(response.into_body()).await,
            );
        }

        #[tokio::test]
        async fn success_web_proof() {
            let helper = TestHelper::create().await;
            let call_data = helper
                .contract
                .web_proof(WebProof {
                    web_proof_json: serde_json::to_string(&json!(load_web_proof_fixture(
                        "../../../web_proof/testdata/tls_proof.json",
                        NOTARY_PUB_KEY_PEM_EXAMPLE
                    )))
                    .unwrap(),
                })
                .calldata()
                .unwrap();

            let req = json!({
                "method": "v_call",
                "params": [
                    {
                        "to": helper.contract.address(),
                        "data": call_data
                    },
                    {
                        "chain_id": 11155111
                    }
                    ],
                "id": 1,
                "jsonrpc": "2.0",
            });

            let response = helper.post("/", &req).await;

            assert_eq!(StatusCode::OK, response.status());
            assert_json_include!(
                expected: json!({
                        "jsonrpc": "2.0",
                        "id": 1,
                        "result": {
                            "evm_call_result": Uint8::from(1).encode_hex(),
                            "proof": {
                                "length": 160,
                                "seal": {
                                    "verifierSelector": "0xdeafbeef",
                                    "mode": 1,
                                },
                                "commitment": {
                                    "functionSelector": function_selector(&call_data),
                                    "proverContractAddress": helper.contract.address(),
                                }
                            },
                        }
                    }),
                actual: body_to_json(response.into_body()).await,
            );
        }
    }
}

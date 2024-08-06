use axum::http::StatusCode;
use axum_jrpc::{JsonRpcRequest, Value};
use core::str;
use serde_json::json;
use server::server::{server, Config};

mod test_helpers;

use test_helpers::{anvil::test_helper, body_to_json, body_to_string, post};

mod server_tests {
    use super::*;

    #[tokio::test]
    async fn http_not_found() -> anyhow::Result<()> {
        let test_helper = test_helper().await;
        let app = server(Config {
            url: test_helper.anvil().endpoint(),
            port: 3000,
        });
        let response = post(app, "/non_existent_http_path", &()).await?;

        assert_eq!(StatusCode::NOT_FOUND, response.status());
        assert!(body_to_string(response.into_body()).await?.is_empty());

        Ok(())
    }

    const EXAMPLE_SMART_CONTRACT_ADDRESS: &str = "5fbdb2315678afecb367f032d93f642f64180aa3";
    const SUM_TEST_DATA: &str = "0xcad0899b00000000000000000000000000000000000000000000000000000000000000010000000000000000000000000000000000000000000000000000000000000002";
    const WEB_PROOF_TEST_DATA: &str = "0xe752d2a000000000000000000000000000000000000000000000000000000000000000200000000000000000000000000000000000000000000000000000000000000020000000000000000000000000000000000000000000000000000000000000000375726c0000000000000000000000000000000000000000000000000000000000";

    #[tokio::test]
    async fn json_rpc_not_found() -> anyhow::Result<()> {
        let test_helper = test_helper().await;
        let app = server(Config {
            url: test_helper.anvil().endpoint(),
            port: 3000,
        });

        let req = JsonRpcRequest {
            method: "non_existent_json_rpc_method".to_string(),
            params: Value::Null,
            id: 1.into(),
        };
        let response = post(app, "/", &req).await?;

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
            body_to_json::<Value>(response.into_body()).await?
        );

        Ok(())
    }

    mod v_call {
        use super::*;
        use web_proof::fixtures::{tls_proof_example, NOTARY_PUB_KEY_PEM_EXAMPLE};

        #[tokio::test]
        async fn field_validation_error() -> anyhow::Result<()> {
            let test_helper = test_helper().await;
            let app = server(Config {
                url: test_helper.anvil().endpoint(),
                port: 3000,
            });

            let req = json!({
                "method": "v_call",
                "params": [{"to": "I am not a valid address!", "data": SUM_TEST_DATA}, {"block_no": 0}],
                "id": 1,
                "jsonrpc": "2.0",
            });
            let response = post(app, "/", &req).await?;

            assert_eq!(StatusCode::OK, response.status());
            assert_eq!(
                json!({
                    "jsonrpc": "2.0",
                    "id": 1,
                    "error": {
                        "code": -32602,
                        "message": "Invalid field `to`: Odd number of digits `I am not a valid address!`",
                        "data": null
                    }
                }),
                body_to_json::<Value>(response.into_body()).await?
            );

            Ok(())
        }

        #[tokio::test]
        async fn success_simple_contract_call() -> anyhow::Result<()> {
            let test_helper = test_helper().await;
            let block_nr = test_helper.block_number;
            let app = server(Config {
                url: test_helper.anvil().endpoint(),
                port: 3000,
            });

            let req = json!({
                "method": "v_call",
                "params": [{"to": EXAMPLE_SMART_CONTRACT_ADDRESS, "data": SUM_TEST_DATA}, {"block_no": block_nr, "chain_id": 11155111}],
                "id": 1,
                "jsonrpc": "2.0",
            });
            let response = post(app, "/", &req).await?;

            assert_eq!(StatusCode::OK, response.status());
            assert_eq!(
                json!({
                    "jsonrpc": "2.0",
                    "id": 1,
                    "result": {
                        "result": "prover_contract_address: 0x5FbDB2315678afecb367f032d93F642f64180aa3, function_selector: 0xcad0899b, evm_call_result: [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 3], seal: []"
                    }
                }),
                body_to_json::<Value>(response.into_body()).await?
            );

            Ok(())
        }

        #[tokio::test]
        async fn failed_web_tls_proof_parsing() -> anyhow::Result<()> {
            let test_helper = test_helper().await;
            let block_nr = test_helper.block_number;
            let app = server(Config {
                url: test_helper.anvil().endpoint(),
                port: 3000,
            });

            let req = json!({
                "method": "v_call",
                "params": [
                    {"to": EXAMPLE_SMART_CONTRACT_ADDRESS, "data": SUM_TEST_DATA},
                    {"block_no": block_nr, "chain_id": 11155111},
                    {"web_proof": {
                        "notary_pub_key": NOTARY_PUB_KEY_PEM_EXAMPLE,
                        "tls_proof": "<tls proof value>",
                    }}
                    ],
                "id": 1,
                "jsonrpc": "2.0",
            });
            let response = post(app, "/", &req).await?;

            assert_eq!(StatusCode::OK, response.status());
            assert_eq!(
                json!({
                    "jsonrpc": "2.0",
                    "id": 1,
                    "error": {
                        "code": -32602,
                        "message": "invalid type: string \"<tls proof value>\", expected struct TlsProof",
                        "data": null
                    }
                }),
                body_to_json::<Value>(response.into_body()).await?
            );

            Ok(())
        }

        #[tokio::test]
        async fn failed_notary_pub_key_parsing() -> anyhow::Result<()> {
            let test_helper = test_helper().await;
            let block_nr = test_helper.block_number;
            let app = server(Config {
                url: test_helper.anvil().endpoint(),
                port: 3000,
            });

            let req = json!({
                "method": "v_call",
                "params": [
                    {"to": EXAMPLE_SMART_CONTRACT_ADDRESS, "data": SUM_TEST_DATA},
                    {"block_no": block_nr, "chain_id": 11155111},
                    {"web_proof": {
                        "notary_pub_key": "<notary pub key value>",
                        "tls_proof": "<tls proof value>",
                    }}
                    ],
                "id": 1,
                "jsonrpc": "2.0",
            });
            let response = post(app, "/", &req).await?;

            assert_eq!(StatusCode::OK, response.status());
            assert_eq!(
                json!({
                    "jsonrpc": "2.0",
                    "id": 1,
                    "error": {
                        "code": -32602,
                        "message": "ASN.1 error: PEM error: PEM preamble contains invalid data (NUL byte)",
                        "data": null
                    }
                }),
                body_to_json::<Value>(response.into_body()).await?
            );

            Ok(())
        }

        #[tokio::test]
        async fn success_web_proof() -> anyhow::Result<()> {
            let test_helper = test_helper().await;
            let block_nr = test_helper.block_number;
            let app = server(Config {
                url: test_helper.anvil().endpoint(),
                port: 3000,
            });

            let req = json!({
                "method": "v_call",
                "params": [
                    {"to": EXAMPLE_SMART_CONTRACT_ADDRESS, "data": WEB_PROOF_TEST_DATA},
                    {"block_no": block_nr, "chain_id": 11155111},
                    {"web_proof": {
                        "notary_pub_key": NOTARY_PUB_KEY_PEM_EXAMPLE,
                        "tls_proof": tls_proof_example(),
                    }}
                    ],
                "id": 1,
                "jsonrpc": "2.0",
            });

            let response = post(app, "/", &req).await?;

            assert_eq!(StatusCode::OK, response.status());
            assert_eq!(
                json!({
                    "jsonrpc": "2.0",
                    "id": 1,
                    "result": {
                        "result": "prover_contract_address: 0x5FbDB2315678afecb367f032d93F642f64180aa3, function_selector: 0xe752d2a0, evm_call_result: [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1], seal: []"
                    }
                }),
                body_to_json::<Value>(response.into_body()).await?
            );

            Ok(())
        }
    }
}

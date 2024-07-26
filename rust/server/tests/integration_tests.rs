use axum::http::StatusCode;
use axum_jrpc::{JsonRpcRequest, Value};
use core::str;
use lazy_static::lazy_static;
use serde_json::json;
use server::server::{server, Config};

mod test_helpers;

use test_helpers::{body_to_json, body_to_string, post};

mod server_tests {
    use super::*;

    lazy_static! {
        static ref CONFIG: Config = Config {
            url: "http://localhost:8545".to_string(),
            port: 3000
        };
    }

    #[tokio::test]
    async fn http_not_found() -> anyhow::Result<()> {
        let app = server(CONFIG.clone());
        let response = post(app, "/non_existent_http_path", &()).await?;

        assert_eq!(response.status(), StatusCode::NOT_FOUND);
        assert!(body_to_string(response.into_body()).await?.is_empty());

        Ok(())
    }

    const SIMPLE_SMART_CONTRACT_ADDRESS: &str = "5fbdb2315678afecb367f032d93f642f64180aa3";
    const WEB_PROOF_SMART_CONTRACT_ADDRESS: &str = "e7f1725e7734ce288f8367e1bb143e90bb3f0512";
    const SIMPLE_EXAMPLE_TEST_DATA: &str = "0xcad0899b00000000000000000000000000000000000000000000000000000000000000010000000000000000000000000000000000000000000000000000000000000002";
    const WEB_PROOF_EXAMPLE_TEST_DATA: &str = "0xefedf8100000000000000000000000000000000000000000000000000000000000000020000000000000000000000000000000000000000000000000000000000000002000000000000000000000000000000000000000000000000000000000000000096170692e782e636f6d0000000000000000000000000000000000000000000000";

    fn create_request(method: &str, params: Value) -> Value {
        json!({
            "jsonrpc": "2.0",
            "method": method,
            "params": params,
            "id": 1
        })
    }

    fn create_result(message: &str) -> Value {
        json!({
            "jsonrpc": "2.0",
            "id": 1,
            "result": {
                "result": message
            }
        })
    }

    fn create_error_response(code: i64, message: &str) -> Value {
        json!({
            "jsonrpc": "2.0",
            "id": 1,
            "error": {
                "code": code,
                "message": message,
                "data": null
            }
        })
    }

    #[tokio::test]
    async fn json_rpc_not_found() -> anyhow::Result<()> {
        let app = server(CONFIG.clone());

        let req = create_request("non_existent_json_rpc_method", json!(null));
        let response = post(app, "/", &req).await?;

        assert_eq!(response.status(), StatusCode::OK);
        assert_eq!(
            body_to_json::<Value>(response.into_body()).await?,
            create_error_response(-32601, "Method `non_existent_json_rpc_method` not found")
        );

        Ok(())
    }

    mod v_call {
        use super::*;
        use web_proof::fixtures::{tls_proof_example, NOTARY_PUB_KEY_PEM_EXAMPLE};
        const LOCALHOST_RPC_URL: &str = "http://localhost:8545";

        fn simple_call_example() -> Value {
            json!({"to": SIMPLE_SMART_CONTRACT_ADDRESS, "data": SIMPLE_EXAMPLE_TEST_DATA})
        }

        fn web_proof_call_example() -> Value {
            json!({"to": WEB_PROOF_SMART_CONTRACT_ADDRESS, "data": WEB_PROOF_EXAMPLE_TEST_DATA})
        }

        fn context_example(block_nr: u32) -> Value {
            json!({"block_no": block_nr, "chain_id": 11155111})
        }

        fn web_augmentor_example() -> Value {
            json!({"web_proof": {
                "notary_pub_key": NOTARY_PUB_KEY_PEM_EXAMPLE,
                "tls_proof": tls_proof_example(),
            }})
        }

        async fn get_block_nr() -> u32 {
            let req = create_request("eth_blockNumber", json!([]));

            let response = reqwest::Client::new()
                .post(LOCALHOST_RPC_URL)
                .json(&req)
                .send()
                .await
                .unwrap();

            let body = response.text().await.unwrap();
            let json: Value = serde_json::from_str(&body).unwrap();
            let result = json["result"].clone();
            let result = result.as_str().unwrap();
            u32::from_str_radix(&result[2..], 16).unwrap()
        }

        #[tokio::test]
        async fn field_validation_error() -> anyhow::Result<()> {
            let app = server(CONFIG.clone());

            let req = create_request(
                "v_call",
                json!([{"to": "I am not a valid address!", "data": SIMPLE_EXAMPLE_TEST_DATA}, {"block_no": 0}]),
            );
            let response = post(app, "/", &req).await?;

            assert_eq!(response.status(), StatusCode::OK);
            assert_eq!(
                body_to_json::<Value>(response.into_body()).await?,
                create_error_response(
                    -32602,
                    "Invalid field `to`: Odd number of digits `I am not a valid address!`"
                )
            );

            Ok(())
        }

        #[tokio::test]
        async fn success_simple_contract_call() -> anyhow::Result<()> {
            let block_nr = get_block_nr().await;
            let app = server(CONFIG.clone());

            let req = create_request(
                "v_call",
                json!([simple_call_example(), context_example(block_nr)]),
            );
            let response = post(app, "/", &req).await?;

            assert_eq!(response.status(), StatusCode::OK);
            assert_eq!(
                body_to_json::<Value>(response.into_body()).await?,
                create_result("prover_contract_address: 0x5FbDB2315678afecb367f032d93F642f64180aa3, function_selector: 0xcad0899b, evm_call_result: [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 3], seal: []")
            );

            Ok(())
        }

        #[tokio::test]
        async fn failed_web_tls_proof_parsing() -> anyhow::Result<()> {
            let block_nr = get_block_nr().await;
            let app = server(CONFIG.clone());

            let req = create_request(
                "v_call",
                json!([
                    simple_call_example(),
                    context_example(block_nr),
                {
                    "web_proof": {
                    "notary_pub_key": NOTARY_PUB_KEY_PEM_EXAMPLE,
                    "tls_proof": "<tls proof value>",
                }}
                ]),
            );
            let response = post(app, "/", &req).await?;

            assert_eq!(response.status(), StatusCode::OK);
            assert_eq!(
                body_to_json::<Value>(response.into_body()).await?,
                create_error_response(
                    -32602,
                    "invalid type: string \"<tls proof value>\", expected struct TlsProof"
                )
            );

            Ok(())
        }

        #[tokio::test]
        async fn failed_notary_pub_key_parsing() -> anyhow::Result<()> {
            let block_nr = get_block_nr().await;
            let app = server(CONFIG.clone());

            let req = create_request(
                "v_call",
                json!([
                    simple_call_example(),
                    context_example(block_nr),
                {
                    "web_proof": {
                    "notary_pub_key": "<notary pub key value>",
                    "tls_proof": tls_proof_example(),
                }}
                ]),
            );
            let response = post(app, "/", &req).await?;

            assert_eq!(response.status(), StatusCode::OK);
            assert_eq!(
                body_to_json::<Value>(response.into_body()).await?,
                create_error_response(
                    -32602,
                    "ASN.1 error: PEM error: PEM preamble contains invalid data (NUL byte)"
                )
            );

            Ok(())
        }

        #[tokio::test]
        async fn success_web_proof() -> anyhow::Result<()> {
            let block_nr = get_block_nr().await;
            let app = server(CONFIG.clone());

            let req = create_request(
                "v_call",
                json!([
                    web_proof_call_example(),
                    context_example(block_nr),
                    web_augmentor_example()
                ]),
            );

            let response = post(app, "/", &req).await?;

            assert_eq!(response.status(), StatusCode::OK);
            assert_eq!(
                body_to_json::<Value>(response.into_body()).await?,
                create_result("prover_contract_address: 0xe7f1725E7734CE288F8367e1Bb143E90bb3F0512, function_selector: 0xefedf810, evm_call_result: [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1], seal: []")
            );

            Ok(())
        }
    }
}

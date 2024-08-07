use axum::http::StatusCode;
use serde_json::json;

mod test_helpers;

use test_helpers::{body_to_json, body_to_string, test_helper};

mod server_tests {
    use super::*;

    #[tokio::test]
    async fn http_not_found() -> anyhow::Result<()> {
        let helper = test_helper().await;
        let response = helper.post("/non_existent_http_path", &()).await?;

        assert_eq!(StatusCode::NOT_FOUND, response.status());
        assert!(body_to_string(response.into_body()).await?.is_empty());

        Ok(())
    }

    #[tokio::test]
    async fn json_rpc_not_found() -> anyhow::Result<()> {
        let helper = test_helper().await;

        let req = json!({
            "method": "non_existent_json_rpc_method",
            "params": [],
            "id": 1,
            "jsonrpc": "2.0",
        });
        let response = helper.post("/", &req).await?;

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

        Ok(())
    }

    mod v_call {
        use crate::test_helpers::Web;

        use super::*;
        use ethers::types::U256;
        use web_proof::fixtures::{tls_proof_example, NOTARY_PUB_KEY_PEM_EXAMPLE};

        #[tokio::test]
        async fn field_validation_error() -> anyhow::Result<()> {
            let helper = test_helper().await;

            let req = json!({
                "method": "v_call",
                "params": [
                    {
                        "to": "I am not a valid address!",
                        "data": helper.contract().sum(U256::from(1), U256::from(2)).calldata().unwrap()
                    },
                    {
                        "block_no": 0
                    }
                    ],
                "id": 1,
                "jsonrpc": "2.0",
            });
            let response = helper.post("/", &req).await?;

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
                body_to_json(response.into_body()).await
            );

            Ok(())
        }

        #[tokio::test]
        async fn success_simple_contract_call() -> anyhow::Result<()> {
            let helper = test_helper().await;
            let call_data = helper
                .contract()
                .sum(U256::from(1), U256::from(2))
                .calldata()
                .unwrap();

            let req = json!({
                "method": "v_call",
                "params": [
                    {
                        "to": helper.contract().address(),
                        "data": call_data
                    },
                    {
                        "block_no": helper.block_number,
                        "chain_id": 11155111
                    }
                    ],
                "id": 1,
                "jsonrpc": "2.0",
            });
            let response = helper.post("/", &req).await?;

            assert_eq!(StatusCode::OK, response.status());
            assert_eq!(
                json!({
                    "jsonrpc": "2.0",
                    "id": 1,
                    "result": {
                        "result": {
                            "evm_call_result": [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 3],
                            "function_selector": call_data.to_string()[0..10],
                            "prover_contract_address": helper.contract().address(),
                            "seal": []
                        }
                    }
                }),
                body_to_json(response.into_body()).await
            );

            Ok(())
        }

        #[tokio::test]
        async fn failed_web_tls_proof_parsing() -> anyhow::Result<()> {
            let helper = test_helper().await;

            let req = json!({
                "method": "v_call",
                "params": [
                    {
                        "to": helper.contract().address(),
                        "data": helper.contract().sum(U256::from(1), U256::from(2)).calldata().unwrap()
                    },
                    {
                        "block_no": helper.block_number,
                        "chain_id": 11155111
                    },
                    {
                        "web_proof":
                        {
                            "notary_pub_key": NOTARY_PUB_KEY_PEM_EXAMPLE,
                            "tls_proof": "<tls proof value>",
                        }
                    }
                    ],
                "id": 1,
                "jsonrpc": "2.0",
            });
            let response = helper.post("/", &req).await?;

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
                body_to_json(response.into_body()).await
            );

            Ok(())
        }

        #[tokio::test]
        async fn failed_notary_pub_key_parsing() -> anyhow::Result<()> {
            let helper = test_helper().await;

            let req = json!({
                "method": "v_call",
                "params": [
                    {
                        "to": helper.contract().address(),
                        "data": helper.contract().sum(U256::from(1), U256::from(2)).calldata().unwrap()
                    },
                    {
                        "block_no": helper.block_number,
                        "chain_id": 11155111
                    },
                    {
                        "web_proof":
                        {
                            "notary_pub_key": "<notary pub key value>",
                            "tls_proof": "<tls proof value>",
                        }
                    }
                    ],
                "id": 1,
                "jsonrpc": "2.0",
            });
            let response = helper.post("/", &req).await?;

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
                body_to_json(response.into_body()).await
            );

            Ok(())
        }

        #[tokio::test]
        async fn success_web_proof() -> anyhow::Result<()> {
            let helper = test_helper().await;
            let call_data = helper
                .contract()
                .web_proof(Web {
                    url: "api.x.com".to_string(),
                })
                .calldata()
                .unwrap();

            let req = json!({
                "method": "v_call",
                "params": [
                    {
                        "to": helper.contract().address(),
                        "data": call_data
                    },
                    {
                        "block_no": helper.block_number,
                        "chain_id": 11155111
                    },
                    {
                        "web_proof": {
                            "notary_pub_key": NOTARY_PUB_KEY_PEM_EXAMPLE,
                            "tls_proof": tls_proof_example(),
                        }
                    }
                    ],
                "id": 1,
                "jsonrpc": "2.0",
            });

            let response = helper.post("/", &req).await?;

            assert_eq!(StatusCode::OK, response.status());
            assert_eq!(
                json!({
                    "jsonrpc": "2.0",
                    "id": 1,
                    "result": {
                        "result": {
                            "evm_call_result": [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1],
                            "function_selector": call_data.to_string()[0..10],
                            "prover_contract_address": helper.contract().address(),
                            "seal": []
                        }
                    }
                }),
                body_to_json(response.into_body()).await
            );

            Ok(())
        }
    }
}

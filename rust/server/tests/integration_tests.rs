use axum::http::StatusCode;
use axum_jrpc::{JsonRpcRequest, Value};
use core::str;
use ethers::{
    contract::abigen,
    core::utils::{Anvil, AnvilInstance},
    middleware::SignerMiddleware,
    providers::{Http, Provider},
    signers::{LocalWallet, Signer},
    types::U256,
};
use eyre::Result;
use lazy_static::lazy_static;
use serde_json::json;
use server::server::{server, Config};
use std::{path::PathBuf, sync::Arc, time::Duration};

mod test_helpers;

use test_helpers::{body_to_json, body_to_string, post};

abigen!(
    SimpleProver,
    "../../examples/simple/out/SimpleProver.sol/SimpleProver.json",
);
abigen!(
    WebProofProver,
    "../../examples/web_proof/out/WebProofProver.sol/WebProofProver.json",
);

async fn setup_anvil() -> Result<AnvilInstance> {
    let anvil = Anvil::new().spawn();
    let wallet: LocalWallet = anvil.keys()[0].clone().into();
    let provider =
        Provider::<Http>::try_from(anvil.endpoint())?.interval(Duration::from_millis(10u64));
    let client = Arc::new(SignerMiddleware::new(
        provider,
        wallet.with_chain_id(anvil.chain_id()),
    ));
    let greeter_contract = SimpleProver::deploy(client.clone(), ())
        .unwrap()
        .send()
        .await
        .unwrap();
    WebProofProver::deploy(client.clone(), ())
        .unwrap()
        .send()
        .await
        .unwrap();
    Ok(anvil)
}

mod server_tests {
    use super::*;

    #[tokio::test]
    async fn http_not_found() -> anyhow::Result<()> {
        let anvil = setup_anvil().await.unwrap();
        let app = server(Config {
            url: anvil.endpoint(),
            port: 3000,
        });
        let response = post(app, "/non_existent_http_path", &()).await?;

        assert_eq!(response.status(), StatusCode::NOT_FOUND);
        assert!(body_to_string(response.into_body()).await?.is_empty());

        Ok(())
    }

    const SIMPLE_SMART_CONTRACT_ADDRESS: &str = "5fbdb2315678afecb367f032d93f642f64180aa3";
    const WEB_PROOF_SMART_CONTRACT_ADDRESS: &str = "e7f1725e7734ce288f8367e1bb143e90bb3f0512";
    const SIMPLE_EXAMPLE_TEST_DATA: &str = "0xcad0899b00000000000000000000000000000000000000000000000000000000000000010000000000000000000000000000000000000000000000000000000000000002";
    const WEB_PROOF_EXAMPLE_TEST_DATA: &str = "0xefedf8100000000000000000000000000000000000000000000000000000000000000020000000000000000000000000000000000000000000000000000000000000002000000000000000000000000000000000000000000000000000000000000000096170692e782e636f6d0000000000000000000000000000000000000000000000";

    #[tokio::test]
    async fn json_rpc_not_found() -> anyhow::Result<()> {
        let anvil = setup_anvil().await.unwrap();
        let app = server(Config {
            url: anvil.endpoint(),
            port: 3000,
        });

        let req = JsonRpcRequest {
            method: "non_existent_json_rpc_method".to_string(),
            params: Value::Null,
            id: 1.into(),
        };
        let response = post(app, "/", &req).await?;

        assert_eq!(response.status(), StatusCode::OK);
        assert_eq!(
            body_to_json::<Value>(response.into_body()).await?,
            json!({
                "jsonrpc": "2.0",
                "id": 1,
                "error": {
                    "code": -32601,
                    "message": "Method `non_existent_json_rpc_method` not found",
                    "data": null
                }
            })
        );

        Ok(())
    }

    mod v_call {
        use super::*;
        use web_proof::fixtures::{tls_proof_example, NOTARY_PUB_KEY_PEM_EXAMPLE};

        async fn get_block_nr(anvil: &AnvilInstance) -> u32 {
            let req = json!({
                "jsonrpc": "2.0",
                "method": "eth_blockNumber",
                "params": [],
                "id": 0
            });

            let response = reqwest::Client::new()
                .post(anvil.endpoint())
                .json(&req)
                .send()
                .await
                .unwrap();

            let body = response.text().await.unwrap();
            let json: serde_json::Value = serde_json::from_str(&body).unwrap();
            let result = json["result"].clone();
            let result = result.as_str().unwrap();
            u32::from_str_radix(&result[2..], 16).unwrap()
        }

        #[tokio::test]
        async fn field_validation_error() -> anyhow::Result<()> {
            let anvil = setup_anvil().await.unwrap();
            let app = server(Config {
                url: anvil.endpoint(),
                port: 3000,
            });

            let req = json!({
                "method": "v_call",
                "params": [{"to": "I am not a valid address!", "data": SIMPLE_EXAMPLE_TEST_DATA}, {"block_no": 0}],
                "id": 1,
                "jsonrpc": "2.0",
            });
            let response = post(app, "/", &req).await?;

            assert_eq!(response.status(), StatusCode::OK);
            assert_eq!(
                body_to_json::<Value>(response.into_body()).await?,
                json!({
                    "jsonrpc": "2.0",
                    "id": 1,
                    "error": {
                        "code": -32602,
                        "message": "Invalid field `to`: Odd number of digits `I am not a valid address!`",
                        "data": null
                    }
                })
            );

            Ok(())
        }

        #[tokio::test]
        async fn success_simple_contract_call() -> anyhow::Result<()> {
            let anvil = setup_anvil().await.unwrap();
            let block_nr = get_block_nr(&anvil).await;
            let app = server(Config {
                url: anvil.endpoint(),
                port: 3000,
            });

            let req = json!({
                "method": "v_call",
                "params": [{"to": SIMPLE_SMART_CONTRACT_ADDRESS, "data": SIMPLE_EXAMPLE_TEST_DATA}, {"block_no": block_nr, "chain_id": 11155111}],
                "id": 1,
                "jsonrpc": "2.0",
            });
            let response = post(app, "/", &req).await?;

            assert_eq!(response.status(), StatusCode::OK);
            assert_eq!(
                body_to_json::<Value>(response.into_body()).await?,
                json!({
                    "jsonrpc": "2.0",
                    "id": 1,
                    "result": {
                        "result": "prover_contract_address: 0x5FbDB2315678afecb367f032d93F642f64180aa3, function_selector: 0xcad0899b, evm_call_result: [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 3], seal: []"
                    }
                })
            );

            Ok(())
        }

        #[tokio::test]
        async fn failed_web_tls_proof_parsing() -> anyhow::Result<()> {
            let anvil = setup_anvil().await.unwrap();
            let block_nr = get_block_nr(&anvil).await;
            let app = server(Config {
                url: anvil.endpoint(),
                port: 3000,
            });

            let req = json!({
                "method": "v_call",
                "params": [
                    {"to": SIMPLE_SMART_CONTRACT_ADDRESS, "data": SIMPLE_EXAMPLE_TEST_DATA},
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

            assert_eq!(response.status(), StatusCode::OK);
            assert_eq!(
                body_to_json::<Value>(response.into_body()).await?,
                json!({
                    "jsonrpc": "2.0",
                    "id": 1,
                    "error": {
                        "code": -32602,
                        "message": "invalid type: string \"<tls proof value>\", expected struct TlsProof",
                        "data": null
                    }
                })
            );

            Ok(())
        }

        #[tokio::test]
        async fn failed_notary_pub_key_parsing() -> anyhow::Result<()> {
            let anvil = setup_anvil().await.unwrap();
            let block_nr = get_block_nr(&anvil).await;
            let app = server(Config {
                url: anvil.endpoint(),
                port: 3000,
            });

            let req = json!({
                "method": "v_call",
                "params": [
                    {"to": SIMPLE_SMART_CONTRACT_ADDRESS, "data": SIMPLE_EXAMPLE_TEST_DATA},
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

            assert_eq!(response.status(), StatusCode::OK);
            assert_eq!(
                body_to_json::<Value>(response.into_body()).await?,
                json!({
                    "jsonrpc": "2.0",
                    "id": 1,
                    "error": {
                        "code": -32602,
                        "message": "ASN.1 error: PEM error: PEM preamble contains invalid data (NUL byte)",
                        "data": null
                    }
                })
            );

            Ok(())
        }

        #[tokio::test]
        async fn success_web_proof() -> anyhow::Result<()> {
            let anvil = setup_anvil().await.unwrap();
            let block_nr = get_block_nr(&anvil).await;
            let app = server(Config {
                url: anvil.endpoint(),
                port: 3000,
            });

            let req = json!({
                "method": "v_call",
                "params": [
                    {"to": WEB_PROOF_SMART_CONTRACT_ADDRESS, "data": WEB_PROOF_EXAMPLE_TEST_DATA},
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

            assert_eq!(response.status(), StatusCode::OK);
            assert_eq!(
                body_to_json::<Value>(response.into_body()).await?,
                json!({
                    "jsonrpc": "2.0",
                    "id": 1,
                    "result": {
                        "result": "prover_contract_address: 0xe7f1725E7734CE288F8367e1Bb143E90bb3F0512, function_selector: 0xefedf810, evm_call_result: [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1], seal: []"
                    }
                })
            );

            Ok(())
        }
    }
}
